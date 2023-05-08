use std::net::SocketAddr;

use crate::AppState;

#[derive(Debug, thiserror::Error)]
pub enum GrpcServerError {}

pub async fn run(addr: SocketAddr, _state: AppState) -> Result<(), GrpcServerError> {
    tracing::debug!("grpc listening on {}", addr);
    // tonic::transport::Server::builder()
    //     .serve(addr)
    //     .await
    //     .unwrap();

    #[allow(clippy::empty_loop)]
    loop {}

    #[allow(unreachable_code)]
    Ok(())
}
