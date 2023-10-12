use std::net::SocketAddr;

use crate::State;

use super::BindError;

/// Tonic-generated gRPC bindings
pub mod authcore {
    tonic::include_proto!("authcore");
}

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
pub async fn run(addr: SocketAddr, _state: State) -> Result<(), BindError> {
    tracing::info!("grpc listening on {}", addr);
    // tonic::transport::Server::builder()
    //     .serve(addr)
    //     .await
    //     .map_err(|e| BindError::GRPC(GrpcServerError::Tonic(e)))?;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    #[allow(unreachable_code)]
    Ok(())
}
