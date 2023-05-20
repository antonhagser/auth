//! This module defines the server's global state, which is shared across
//! all routes and services.

use std::sync::Arc;

use crypto::snowflake::SnowflakeGenerator;

use crate::models::PrismaClient;

/// `State` represents the server's global state.
#[derive(Debug)]
pub struct State {
    prisma: PrismaClient,
    id_generator: SnowflakeGenerator,
}

impl State {
    pub fn new(prisma: PrismaClient, id_generator: SnowflakeGenerator) -> Self {
        Self {
            prisma,
            id_generator,
        }
    }

    pub fn prisma(&self) -> &PrismaClient {
        &self.prisma
    }

    pub fn id_generator(&self) -> &SnowflakeGenerator {
        &self.id_generator
    }
}

/// `AppState` is an alias for an `Arc<State>` to provide shared ownership
/// and thread-safe access to the server's global state.
pub type AppState = Arc<State>;
