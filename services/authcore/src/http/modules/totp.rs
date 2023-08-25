//! TOTP Module
//!
//! This module provides the functionality for TOTP (Time-based One-time Password) authentication.

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

/// Module for handling verification of TOTP codes and authentication.
pub mod verify;

/// Module for handling deletion of TOTP.
pub mod delete;

/// Module for handling setup of TOTP.
pub mod setup;

/// Module for handling backup codes of TOTP.
pub mod backup_codes;

/// Router for handling routing within totp.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/verify", post(verify::route))
        // TODO: Place endpoints behind authentication middleware (using Authorization header)
        .route("/delete", post(delete::route))
        .route("/setup", post(setup::route))
        .route("/backup_codes", get(backup_codes::route))
        .with_state(state)
}
