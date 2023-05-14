//! # HTTP modules
//! Contains the HTTP modules for the AuthCore service.
//!
//! ## Modules
//! - [`basic_auth`](basic_auth/index.html): Basic authentication module.

use crate::state::AppState;

pub mod basic_auth;

/// Verification submodule for handling user verification.
pub mod verification;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .with_state(state.clone())
        .nest("/", basic_auth::router(state.clone()))
        .nest("/", verification::router(state))
}
