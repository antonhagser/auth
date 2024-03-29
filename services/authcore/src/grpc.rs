//! This module defines a gRPC server
//! and currently features a placeholder implementation.

use std::net::SocketAddr;

use crate::AppState;

mod auth;
mod error;
mod platform;
mod session;

pub mod client;

/// Tonic-generated gRPC bindings
pub mod authcore {
    tonic::include_proto!("authcore");
}

/// GrpcServerError enumerates the possible errors that can occur
/// when running the gRPC server. As the current implementation is a placeholder,
/// there are no variants defined.
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerError {
    #[error("gRPC server error")]
    GRPCServerError(tonic::transport::Error),
}

/// Starts and runs the gRPC server on the given address with the provided state.
///
/// # Arguments
///
/// * `addr`: The socket address to bind the server to.
/// * `state`: The shared application state to use across all routes.
///
/// # Errors
///
/// Currently, this function cannot return any errors as the implementation
/// is a placeholder.
pub async fn run(addr: SocketAddr, state: AppState) -> Result<(), GrpcServerError> {
    let inner = platform::PlatformServer::new(state.clone());
    let svc_platform = authcore::platform_server::PlatformServer::new(inner);

    let inner = auth::basic::BasicServer::new(state.clone());
    let svc_basic = auth::basic::basic_auth_server::BasicAuthServer::new(inner);

    let inner = session::SessionServer::new(state.clone());
    let svc_session = session::session_server::SessionServer::new(inner);

    tracing::info!("grpc listening on {}", addr);
    tonic::transport::Server::builder()
        .add_service(svc_platform)
        .add_service(svc_basic)
        .add_service(svc_session)
        .serve(addr)
        .await
        .map_err(GrpcServerError::GRPCServerError)?;

    #[allow(unreachable_code)]
    Ok(())
}
