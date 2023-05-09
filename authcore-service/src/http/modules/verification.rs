use axum::Router;

use crate::state::AppState;

/// Router for handling routing within verification.
pub fn router(state: AppState) -> Router {
    Router::new().with_state(state)
}
