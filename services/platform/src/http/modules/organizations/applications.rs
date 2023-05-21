use crate::state::AppState;

pub mod create;
pub mod delete;
pub mod get;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new().with_state(state)
}
