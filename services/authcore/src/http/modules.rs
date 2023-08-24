//! # HTTP modules
//! Contains the HTTP modules for the AuthCore service.
//!
//! ## Modules
//! - [`basic_auth`](basic_auth/index.html): Basic authentication module.

use hyper::{http::request::Parts, Body};
use serde::de::DeserializeOwned;

use crate::state::AppState;

pub mod basic;
pub mod totp;

/// Verification submodule for handling user verification.
pub mod verification;

/// Router submodule for handling routing.
pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .with_state(state.clone())
        .nest("/basic", basic::router(state.clone()))
        .nest("/verify", verification::router(state.clone()))
        .nest("/totp", totp::router(state))
}

async fn get_request<T>(parts: &Parts, body: Body) -> Option<T>
where
    T: DeserializeOwned,
{
    match parts
        .headers
        .get("content-type")
        .and_then(|header| header.to_str().ok())
    {
        Some("application/x-www-form-urlencoded") => {
            let body = hyper::body::to_bytes(body).await.ok()?;
            let value = serde_urlencoded::from_bytes(&body).ok()?;

            Some(value)
        }
        Some("application/json") => {
            let body = hyper::body::to_bytes(body).await.ok()?;
            let value = serde_json::from_slice(&body).ok()?;

            Some(value)
        }
        _ => None,
    }
}
