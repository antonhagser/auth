use axum::{extract::State, Json};
use hyper::StatusCode;

use crate::state::AppState;

pub async fn route(State(_state): State<AppState>) -> (StatusCode, Json<()>) {
    (StatusCode::OK, Json(()))
}
