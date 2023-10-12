#![forbid(unsafe_code)]

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use api::{
    grpc::{self, GrpcServerError},
    http::{self, HTTPServerError},
};
use futures::future::select;
use once_cell::sync::Lazy;
use tokio::task::JoinHandle;
use tracing::info;

use state::AppState;

use crate::state::State;

pub mod api;
pub mod database;
pub mod state;

static DATABASE_URL: Lazy<String> =
    once_cell::sync::Lazy::new(|| std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    info!("starting service");

    // Initialize the database pool
    let pool = database::init_pool().await?;

    // Run database migrations
    database::migrate(&pool).await?;

    // Initialize the application state
    let state = Arc::new(AppState { pool });

    // If we are running in debug mode, bind to localhost
    let ip = if cfg!(debug_assertions) {
        tracing::info!("running in debug mode");
        Ipv4Addr::LOCALHOST
    } else {
        tracing::info!("running in release mode");
        Ipv4Addr::UNSPECIFIED
    };

    // Bind to the default ports
    let http_addr = SocketAddr::from((ip, 8080));
    let grpc_addr = SocketAddr::from((ip, 58080));

    // Start the gRPC server
    api::bind(state.clone(), http_addr, grpc_addr).await?;

    info!("service stopped");
    Ok(())
}
