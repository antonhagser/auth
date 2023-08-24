//! BasicAuth Module
//!
//! This module provides the functionality for traditional password-based authentication.
//! It includes both the login and registration processes.

use axum::{routing::post, Router};

use crate::state::AppState;

/// Login submodule for handling user authentication using a username/email and password.
pub mod login;

/// Register submodule for handling user registration, including user creation and password storage.
pub mod register;

/// Router for handling routing within basic_auth.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login::route))
        .route("/register", post(register::route))
        .with_state(state)
}
