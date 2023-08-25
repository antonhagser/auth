//! BasicAuth Module
//!
//! This module provides the functionality for traditional password-based authentication.
//! It includes both the login and registration processes.

use axum::{routing::post, Router};

use crate::state::AppState;

pub mod refresh;

/// Router for handling routing within session.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/refresh", post(refresh::route))
        .with_state(state)
}
