use crate::state::AppState;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new().with_state(state)
}
