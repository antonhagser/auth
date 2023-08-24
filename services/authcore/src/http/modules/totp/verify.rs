use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use crypto::tokens::jsonwebtoken::Claims;
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    core::{basic::login, totp},
    http::{modules::get_request, response::HTTPResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct VerifyRequest {
    token: String,
    totp_code: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn route(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
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

            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    // Verify the totp flow token
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
        Some(addr.ip().to_string()),
        user_agent,
    )
    .await
    {
        Ok(c) => c,
        Err(e) => match e {
            totp::VerifyFlowTokenError::Expired => {
                return (
                    StatusCode::UNAUTHORIZED,
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
            Json(HTTPResponse::error(
                "InternalServerError",
                "Could not verify totp".to_owned(),
                (),
            )),
        );
    }

    info!("Verified totp flow token and user, ok to proceed");

    // Match totp code
    let totp = user.totp().unwrap().verify(data.totp_code);
    if !totp {
        return (
            StatusCode::UNAUTHORIZED,
            Json(HTTPResponse::error(
                "Unauthorized",
                "Invalid totp code".to_owned(),
                (),
            )),
        );
    }

    // Generate refresh and access token
    let (user_token, refresh_token) =
        match login::create_refresh_and_access_token(&state, &prisma_client, &user).await {
            Ok(tokens) => tokens,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
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
            Json(HTTPResponse::error(
                "InternalServerError",
                "Could not verify totp".to_owned(),
                (),
            )),
        );
    }

    // Return the access token and refresh token to the client
    let response = VerifyResponse {
        access_token: user_token.token().into(),
        refresh_token,
    };

    (StatusCode::OK, Json(HTTPResponse::ok(response)))
}
