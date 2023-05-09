use axum::Json;
use hyper::StatusCode;

pub async fn route() -> (StatusCode, Json<()>) {
    (StatusCode::OK, Json(()))
}
