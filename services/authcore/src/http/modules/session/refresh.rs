use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use axum_extra::extract::CookieJar;
use crypto::snowflake::Snowflake;
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    core::token::{self, get_refresh_cookie},
    http::{modules::get_request, response::HTTPResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct RefreshRequest {
    application_id: Snowflake,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    access: String,
}

pub async fn route(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
    let (parts, body) = request.into_parts();

    // Accept multiple different ways to give the data, url encoded form data or json body
    let data: RefreshRequest = match get_request(&parts, body).await {
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

    // Get refresh token from cookie
    let refresh = get_refresh_cookie(&jar, data.application_id);
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
