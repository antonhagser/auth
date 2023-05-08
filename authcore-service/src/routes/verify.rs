use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct VerifyResponse {
    is_valid: bool,
    expired: bool,
}

pub async fn verify(
    State(_state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> (StatusCode, Json<VerifyResponse>) {
    let _token = authorization.token();

    (
        StatusCode::UNAUTHORIZED,
        Json(VerifyResponse {
            is_valid: false,
            expired: true,
        }),
    )
}
