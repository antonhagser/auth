use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use chrono::{Duration, Utc};
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    core::{
        basic::{login, login::BasicLoginData},
        token,
    },
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
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn route(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
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

            return (StatusCode::BAD_REQUEST, Json(response));
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

            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    let login_data = BasicLoginData {
        email: data.email,
        password: data.password,
        application_id,
        ip_address: addr.ip().to_string(),
        user_agent: parts
            .headers
            .get("user-agent")
            .map(|v| v.to_str().unwrap_or_default().to_owned())
            .unwrap_or_default(),
        totp_code: data.totp_code,
    };

    let user = match login::with_basic_auth(&state, login_data).await {
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
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
                    }
                };

                let response = HTTPResponse::error(
                    "NeedFurtherVerificationThrough2FA",
                    "The user needs to verify their identity through 2FA".to_owned(),
                    flow_token,
                );

                return (StatusCode::UNAUTHORIZED, Json(response));
            }
            _ => {
                let response =
                    HTTPResponse::error("Unauthorized", "Invalid email or password".to_owned(), ());

                return (StatusCode::UNAUTHORIZED, Json(response));
            }
        },
    };

    let (transaction, transaction_client) = state.prisma()._transaction().begin().await.unwrap();

    // Generate a new user refresh token
    let refresh_token = token::new_refresh_token(
        &transaction_client,
        state.id_generator(),
        user.id(),
        Utc::now() + Duration::days(30),
    )
    .await;

    let refresh_token = match refresh_token {
        Ok(refresh_token) => refresh_token,
        Err(e) => {
            error!("Failed to generate refresh token: {}", e);
            let _ = transaction.rollback(transaction_client).await;

            let response = HTTPResponse::error("InternalServerError", "A refresh token could not be created for the account due to an internal server error.", ());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // Generate access token
    let access_token = token::new_access_token(
        &state,
        user.id(),
        Utc::now() + Duration::hours(1),
        refresh_token.id(),
    );

    let access_token = match access_token {
        Ok(access_token) => access_token,
        Err(e) => {
            error!("Failed to generate access token: {}", e);
            let _ = transaction.rollback(transaction_client).await;

            let response = HTTPResponse::error("InternalServerError", "An access token could not be created for the account due to an internal server error.", ());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // Commit the transaction
    if transaction.commit(transaction_client).await.is_err() {
        let response = HTTPResponse::error("InternalServerError", "", ());
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }

    // Return the access token and refresh token to the client
    let response = LoginResponse {
        access_token,
        refresh_token: refresh_token.token().into(),
    };

    let response = HTTPResponse::ok(response);
    (StatusCode::OK, Json(response))
}
