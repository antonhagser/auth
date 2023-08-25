use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use crypto::tokens::jsonwebtoken::Claims;
use hyper::{Body, Request, StatusCode};
use serde::Deserialize;

use crate::{
    core::{basic::login, totp},
    http::{
        modules::{basic::login::LoginResponse, get_request},
        response::HTTPResponse,
    },
    state::AppState,
};

#[derive(Deserialize)]
pub struct VerifyRequest {
    token: String,
    totp_code: String,
}

pub async fn route(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request<Body>,
) -> impl IntoResponse {
    let (parts, body) = request.into_parts();

    // Accept multiple different ways to give the data, url encoded form data or json body
    let data: VerifyRequest = match get_request(&parts, body).await {
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

    // Check length of totp code
    if data.totp_code.len() < 6 || data.totp_code.len() > 9 {
        let response = HTTPResponse::error(
            "BadRequest",
            "Invalid totp code, expected 6 digits or 9 characters for backup code".to_owned(),
            (),
        );

        return (StatusCode::BAD_REQUEST, jar, Json(response));
    }

    // Get user agent (used to verify totp flow token)
    let user_agent = parts
        .headers
        .get("user-agent")
        .map(|v| v.to_str().unwrap().to_owned());

    // Begin a transaction
    let (transaction_controller, prisma_client) = match state.prisma()._transaction().begin().await
    {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar,
                Json(HTTPResponse::error(
                    "InternalServerError",
                    "Could not verify totp".to_owned(),
                    (),
                )),
            );
        }
    };

    // Verify the totp flow token
    let claims = match totp::verify_totp_flow_token(
        &state,
        data.token,
        None,
        None,
        user_agent.clone(),
    )
    .await
    {
        Ok(c) => c,
        Err(e) => match e {
            totp::VerifyFlowTokenError::Expired => {
                return (
                    StatusCode::UNAUTHORIZED,
                    jar,
                    Json(HTTPResponse::error(
                        "Expired",
                        "TOTP flow token is expired".to_owned(),
                        (),
                    )),
                );
            }
            _ => {
                return (
                    StatusCode::UNAUTHORIZED,
                    jar,
                    Json(HTTPResponse::error(
                        "Invalid",
                        "TOTP flow token is invalid".to_owned(),
                        (),
                    )),
                );
            }
        },
    };

    // Fetch user from database
    let user = match crate::models::user::User::get(
        &prisma_client,
        claims.sub().try_into().unwrap(),
        vec![crate::models::user::UserWith::TOTP],
    )
    .await
    {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar,
                Json(HTTPResponse::error(
                    "InternalServerError",
                    "Could not verify totp".to_owned(),
                    (),
                )),
            );
        }
    };

    // Check that user has totp
    if user.totp().is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            jar,
            Json(HTTPResponse::error(
                "InternalServerError",
                "Could not verify totp".to_owned(),
                (),
            )),
        );
    }

    let totp = user.totp().take().unwrap();

    // Match totp code
    let totp_result = totp.verify(&prisma_client, data.totp_code).await;
    if totp_result.is_err() || !totp_result.unwrap() {
        return (
            StatusCode::UNAUTHORIZED,
            jar,
            Json(HTTPResponse::error(
                "Unauthorized",
                "Invalid totp code".to_owned(),
                (),
            )),
        );
    }

    // Generate refresh and access token
    let (refresh_token, access_token) = match login::create_refresh_and_access_token(
        &state,
        &prisma_client,
        &user,
        Some(addr.ip().to_string()),
        user_agent,
    )
    .await
    {
        Ok(tokens) => tokens,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar,
                Json(HTTPResponse::error(
                    "InternalServerError",
                    "Could not generate refresh and access token".to_owned(),
                    (),
                )),
            );
        }
    };

    if transaction_controller.commit(prisma_client).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            jar,
            Json(HTTPResponse::error(
                "InternalServerError",
                "Could not verify totp".to_owned(),
                (),
            )),
        );
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
