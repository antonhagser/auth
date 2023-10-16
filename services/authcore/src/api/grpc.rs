use std::net::SocketAddr;

use crate::{
    api::grpc::platform::{authcore::platform_server::PlatformServer, PlatformService},
    State,
};

use super::BindError;

mod platform;

/// GrpcServerError enumerates the possible errors that can occur
/// when running the gRPC server. As the current implementation is a placeholder,
/// there are no variants defined.
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerError {
    #[error("tonic server error")]
    Tonic(tonic::transport::Error),
}

/// Starts and runs the gRPC server on the given address with the provided state.
///
/// # Arguments
///
/// * `addr`: The socket address to bind the server to.
/// * `state`: The shared application state to use across all routes.
pub async fn run(addr: SocketAddr, state: State) -> Result<(), BindError> {
    tracing::info!("grpc listening on {}", addr);

    let platform_svc = PlatformService {
        state: state.clone(),
    };

    tonic::transport::Server::builder()
        .add_service(PlatformServer::new(platform_svc))
        .serve(addr)
        .await
        .map_err(|e| BindError::GRPC(GrpcServerError::Tonic(e)))?;

    Ok(())
}
