//! This module defines the server's global state, which is shared across
//! all routes and services.

use std::sync::Arc;

use crypto::snowflake::SnowflakeGenerator;

use crate::{models::PrismaClient, ServiceData};

pub use config::{Config, CONFIG};

mod config;

/// `State` represents the server's global state.
#[derive(Debug)]
pub struct State {
    prisma_client: PrismaClient,
    id_generator: SnowflakeGenerator,
    service_data: ServiceData,
}

impl State {
    pub fn new(
        prisma_client: PrismaClient,
        id_generator: SnowflakeGenerator,
        service_data: ServiceData,
    ) -> Self {
        Self {
            prisma_client,
            id_generator,
            service_data,
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
}

/// `AppState` is an alias for an `Arc<State>` to provide shared ownership
/// and thread-safe access to the server's global state.
pub type AppState = Arc<State>;
