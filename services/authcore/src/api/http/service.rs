use axum::response::IntoResponse;

pub async fn health() -> impl IntoResponse {
    (hyper::StatusCode::OK, "OK")
}

pub async fn ready() -> impl IntoResponse {
    (hyper::StatusCode::OK, "OK")
}

pub async fn version() -> impl IntoResponse {
    (hyper::StatusCode::OK, "0.1.0")
}
