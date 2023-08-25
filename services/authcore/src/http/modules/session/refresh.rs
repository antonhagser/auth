use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use axum_extra::extract::CookieJar;
use hyper::{Body, Request, StatusCode};
use serde::Serialize;

use crate::{core::token, http::response::HTTPResponse, state::AppState};

#[derive(Serialize)]
pub struct RefreshResponse {
    access: String,
}

pub async fn route(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    jar: CookieJar,
    _request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
    // Get refresh token from cookie
    let refresh = jar.get("refresh");
    let token = match refresh {
        Some(refresh) => {
            // Fetch refresh token from database
            let token = refresh.value();
            match token::verify_refresh_token(&state, state.prisma(), token).await {
                Ok(token) => token,
                Err(_) => {
                    let response =
                        HTTPResponse::error("Unauthorized", "Invalid refresh token".to_owned(), ());
                    return (StatusCode::UNAUTHORIZED, Json(response));
                }
            }
        }
        None => {
            let response =
                HTTPResponse::error("Unauthorized", "Invalid refresh token".to_owned(), ());
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    // Set access token expire to 1 minute
    let expiration = chrono::Utc::now() + chrono::Duration::minutes(1);

    // Generate new access token
    let access_token = token::new_access_token(&state, token.user_id(), expiration, token.id());
    let access_token = match access_token {
        Ok(access_token) => access_token,
        Err(_) => {
            let response = HTTPResponse::error(
                "InternalServerError",
                "Failed to generate access token".to_owned(),
                (),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let response = RefreshResponse {
        access: access_token,
    };

    (StatusCode::OK, Json(HTTPResponse::ok(response)))
}
