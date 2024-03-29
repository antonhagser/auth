use crate::state::AppState;

pub mod organizations;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .with_state(state.clone())
        .nest("/organizations", organizations::router(state))
}
