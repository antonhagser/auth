use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    core::basic::login,
    http::{modules::get_request, response::HTTPResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub application_id: String,

    pub totp_code: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access: String,
}

pub async fn route(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request<Body>,
) -> impl IntoResponse {
    let (parts, body) = request.into_parts();

    // Accept multiple different ways to give the data, url encoded form data or json body
    let data: LoginRequest = match get_request(&parts, body).await {
        Some(d) => d,
        None => {
            let response = HTTPResponse::error(
                "BadRequest",
                "Invalid content type, expected application/x-www-form-urlencoded or application/json".to_owned(),
                (),
            );

            return (StatusCode::BAD_REQUEST, jar, Json(response));
        }
    };

    // Convert the application ID to a snowflake
    let application_id = match data.application_id.try_into() {
        Ok(id) => id,
        Err(_) => {
            let response = HTTPResponse::error(
                "InvalidApplicationID",
                "Invalid application ID".to_owned(),
                (),
            );

            return (StatusCode::BAD_REQUEST, jar, Json(response));
        }
    };

    // Get the user agent
    let user_agent = parts
        .headers
        .get("user-agent")
        .map(|v| v.to_str().unwrap_or_default().to_owned());

    // Start a transaction
    let (transaction_controller, prisma_client) =
        state.prisma()._transaction().begin().await.unwrap();

    // Call core and try to login with basic auth
    let user = match login::with_basic_auth(
        &prisma_client,
        data.email,
        data.password,
        application_id,
        addr.ip().to_string(),
        data.totp_code,
    )
    .await
    {
        Ok(user) => user,
        Err(e) => match e {
            login::BasicLoginError::NeedFurtherVerificationThrough2FA(user) => {
                // Generate a TOTP flow token
                let flow_token = crate::core::totp::new_totp_flow_token(
                    &state,
                    user.id(),
                    None, // TODO: implement device ID
                    None, // TODO: implement session ID
                    Some(addr.ip().to_string()),
                    Some(
                        parts
                            .headers
                            .get("user-agent")
                            .map(|v| v.to_str().unwrap_or_default().to_owned())
                            .unwrap_or_default(),
                    ),
                )
                .await;

                let flow_token = match flow_token {
                    Ok(flow_token) => flow_token,
                    Err(e) => {
                        error!("Failed to generate TOTP flow token: {}", e);

                        let response = HTTPResponse::error("InternalServerError", "A TOTP flow token could not be created for the account due to an internal server error.", ());
                        return (StatusCode::INTERNAL_SERVER_ERROR, jar, Json(response));
                    }
                };

                let response = HTTPResponse::error(
                    "NeedFurtherVerificationThrough2FA",
                    "The user needs to verify their identity through 2FA".to_owned(),
                    flow_token,
                );

                return (StatusCode::UNAUTHORIZED, jar, Json(response));
            }
            _ => {
                let response =
                    HTTPResponse::error("Unauthorized", "Invalid email or password".to_owned(), ());

                return (StatusCode::UNAUTHORIZED, jar, Json(response));
            }
        },
    };

    // Generate a new user refresh token
    let res = login::create_refresh_and_access_token(
        &state,
        &prisma_client,
        &user,
        Some(addr.ip().to_string()),
        user_agent,
    )
    .await;
    let (refresh_token, access_token) = match res {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("Failed to generate refresh and access token: {}", e);

            let _ = transaction_controller.rollback(prisma_client).await;

            let response = HTTPResponse::error(
                "InternalServerError",
                "Failed to create the correct tokens.",
                (),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, jar, Json(response));
        }
    };

    // Commit the transaction
    if transaction_controller.commit(prisma_client).await.is_err() {
        let response = HTTPResponse::error("InternalServerError", "", ());
        return (StatusCode::INTERNAL_SERVER_ERROR, jar, Json(response));
    }

    // Return the access token and refresh token to the client
    let response = LoginResponse {
        access: access_token,
    };

    // Write refresh to cookie
    let jar = jar.add(login::create_refresh_cookie(
        refresh_token.token().to_string(),
        refresh_token.expires_at(),
    ));

    let response = HTTPResponse::ok(response);
    (StatusCode::OK, jar, Json(response))
}
