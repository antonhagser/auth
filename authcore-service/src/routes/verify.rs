use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use crypto::jwt::JWT;
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct VerifyResponse {
    is_valid: bool,
    expired: bool,
}

pub async fn verify(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> (StatusCode, Json<VerifyResponse>) {
    let token = authorization.token();

    // todo: handle error
    match JWT::verify_token(token, state.authenticator.pub_key_pem()) {
        Ok(_) => (
            StatusCode::OK,
            Json(VerifyResponse {
                is_valid: true,
                expired: false,
            }),
        ),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(VerifyResponse {
                is_valid: false,
                expired: e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature,
            }),
        ),
    }
}
