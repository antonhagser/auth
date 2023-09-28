use axum::{routing::get, Router};

use crate::state::AppState;

pub mod email;

/// Router for handling routing within verification.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/email/:token", get(email::route))
        .with_state(state)
}
