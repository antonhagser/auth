use std::net::SocketAddr;

use crate::AppState;

pub async fn run(_addr: SocketAddr, _state: AppState) {
    // tracing::debug!("grpc listening on {}", addr);
    // tonic::transport::Server::builder()
    //     .serve(addr)
    //     .await
    //     .unwrap();
}
