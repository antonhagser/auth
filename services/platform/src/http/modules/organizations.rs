use crate::state::AppState;

pub mod create;
pub mod delete;
pub mod get;

pub mod applications;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .with_state(state.clone())
        .nest("/application", applications::router(state))
}
