//! This module defines the server's global state, which is shared across
//! all routes and services.

use std::sync::Arc;

use crypto::{
    snowflake::SnowflakeGenerator,
    tokens::paseto::{self, SymmetricKey},
};

use crate::{models::PrismaClient, ServiceData};

pub use config::{Config, CONFIG};

mod config;

/// `State` represents the server's global state.
pub struct State {
    prisma_client: PrismaClient,
    id_generator: SnowflakeGenerator,
    service_data: ServiceData,
    paseto_key: SymmetricKey,
}

impl State {
    pub fn new(
        prisma_client: PrismaClient,
        id_generator: SnowflakeGenerator,
        service_data: ServiceData,
        paseto_key: &[u8],
    ) -> Self {
        let keys = paseto::generate_key(paseto_key);

        Self {
            prisma_client,
            id_generator,
            service_data,
            paseto_key: keys,
        }
    }

    pub fn prisma(&self) -> &PrismaClient {
        &self.prisma_client
    }

    pub fn id_generator(&self) -> &SnowflakeGenerator {
        &self.id_generator
    }

    pub fn config(&self) -> &Config {
        &CONFIG
    }

    pub fn service_data(&self) -> &ServiceData {
        &self.service_data
    }

    pub fn paseto_key(&self) -> &SymmetricKey {
        &self.paseto_key
    }
}

/// `AppState` is an alias for an `Arc<State>` to provide shared ownership
/// and thread-safe access to the server's global state.
pub type AppState = Arc<State>;
