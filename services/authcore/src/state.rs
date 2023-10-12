//! This module defines the server's global state, which is shared across
//! all routes and services.

use std::sync::Arc;

use crate::database::Pool;

/// `AppState` represents the server's global state.
pub struct AppState {
    pub(super) pool: Pool,
}

impl AppState {
    /// Creates a new `AppState` with the given database pool.
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Returns a reference to the database pool.
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}

/// `State` is an alias for an `Arc<StAppStateate>` to provide shared ownership
/// and thread-safe access to the server's global state.
pub type State = Arc<AppState>;
