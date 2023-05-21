//! # HTTP modules
//! Contains the HTTP modules for the AuthCore service.
//!
//! ## Modules
//! - [`basic_auth`](basic_auth/index.html): Basic authentication module.

use crate::state::AppState;

pub mod basic;

/// Verification submodule for handling user verification.
pub mod verification;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .with_state(state.clone())
        .nest("/basic", basic::router(state.clone()))
        .nest("/verify", verification::router(state))
    // .nest("/sso", sso::router(state))
}
