use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use grpc::GrpcServerError;
use http::HTTPServerError;
use once_cell::sync::Lazy;
use serde::Serialize;
use state::AppState;
use tokio::task::JoinHandle;
use tracing::info;
use utoipa::ToSchema;

use crate::state::State;

pub mod core;
pub mod grpc;
pub mod http;
pub mod models;
pub mod state;

#[derive(Clone, Copy, Serialize, ToSchema)]
struct ServiceData {
    service_name: &'static str,
    service_version: &'static str,
}

static SERVICE_DATA: Lazy<ServiceData> = Lazy::new(|| ServiceData {
    service_name: "AuthCore",
    service_version: env!("CARGO_PKG_VERSION"),
});

static DATABASE_URL: Lazy<String> =
    once_cell::sync::Lazy::new(|| std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tracing_subscriber::fmt::init();
    console_subscriber::init();
    info!("starting service");

    // Create a shared app state
    let prisma = models::PrismaClient::_builder()
        .with_url(DATABASE_URL.to_string())
        .build()
        .await?;

    let id_generator = crypto::snowflake::SnowflakeGenerator::new(0, 0);

    let app_state = State::new(prisma, id_generator);
    let app_state = Arc::new(app_state);

    // If we are running in debug mode, bind to localhost
    let ip = if cfg!(debug_assertions) {
        tracing::debug!("running in debug mode");
        Ipv4Addr::LOCALHOST
    } else {
        tracing::debug!("running in release mode");
        Ipv4Addr::UNSPECIFIED
    };

    // Bind to the default ports
    let http_addr = SocketAddr::from((ip, 8080));
    let grpc_addr = SocketAddr::from((ip, 58080));

    // Start the gRPC server
    let grpc_handle = start_grpc(app_state.clone(), grpc_addr).await?;

    // Start the HTTP server
    let http_handle = start_http(app_state, http_addr).await?;

    // Join both handles
    futures::future::select(grpc_handle, http_handle).await;

    info!("service stopped");
    Ok(())
}

async fn start_grpc(
    app_state: AppState,
    grpc_addr: SocketAddr,
) -> Result<JoinHandle<Result<(), GrpcServerError>>, Box<dyn std::error::Error>> {
    let handle = tokio::spawn(async move { grpc::run(grpc_addr, app_state).await });
    Ok(handle)
}

async fn start_http(
    app_state: AppState,
    http_addr: SocketAddr,
) -> Result<JoinHandle<Result<(), HTTPServerError>>, Box<dyn std::error::Error>> {
    let handle = tokio::spawn(async move { http::run(http_addr, app_state).await });
    Ok(handle)
}
