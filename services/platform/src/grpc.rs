//! This module defines a gRPC server
//! and currently features a placeholder implementation.

use std::net::SocketAddr;

use crate::AppState;

/// GrpcServerError enumerates the possible errors that can occur
/// when running the gRPC server. As the current implementation is a placeholder,
/// there are no variants defined.
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerError {}

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
pub async fn run(addr: SocketAddr, _state: AppState) -> Result<(), GrpcServerError> {
    tracing::debug!("grpc listening on {}", addr);
    // tonic::transport::Server::builder()
    //     .add_service(HelloServer::new(HelloService))
    //     .serve(addr)
    //     .await
    //     .unwrap();

    #[allow(clippy::empty_loop)]
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(50)).await;
    }

    #[allow(unreachable_code)]
    Ok(())
}
