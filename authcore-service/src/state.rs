//! This module defines the server's global state, which is shared across
//! all routes and services.

use std::sync::Arc;

/// `State` represents the server's global state.
#[derive(Debug)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// `AppState` is an alias for an `Arc<State>` to provide shared ownership
/// and thread-safe access to the server's global state.
pub type AppState = Arc<State>;
