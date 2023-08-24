use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    http::{modules::get_request, response::HTTPResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct VerifyRequest {}

#[derive(Serialize)]
pub struct VerifyResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn route(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(_state): State<AppState>,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
    let (parts, body) = request.into_parts();

    // Accept multiple different ways to give the data, url encoded form data or json body
    let _data: VerifyRequest = match get_request(&parts, body).await {
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

    (StatusCode::OK, Json(HTTPResponse::empty()))
}
