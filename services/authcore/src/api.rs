use std::net::SocketAddr;

use futures::future::{select, Either};
use tokio::task::JoinHandle;

use crate::state::State;

use self::{grpc::GrpcServerError, http::HTTPServerError};

pub mod grpc;
pub mod http;

async fn start_grpc(state: State, grpc_addr: SocketAddr) -> JoinHandle<Result<(), BindError>> {
    tokio::spawn(async move { grpc::run(grpc_addr, state).await })
}

async fn start_http(state: State, http_addr: SocketAddr) -> JoinHandle<Result<(), BindError>> {
    tokio::spawn(async move { http::run(http_addr, state).await })
}

#[derive(Debug, thiserror::Error)]
pub enum BindError {
    #[error("HTTP server error: {0}")]
    HTTP(HTTPServerError),
    #[error("gRPC server error: {0}")]
    GRPC(GrpcServerError),

    #[error("unknown error")]
    Unknown,
}

pub async fn bind(
    state: State,
    http_addr: SocketAddr,
    grpc_addr: SocketAddr,
) -> Result<(), BindError> {
    let http_handle = start_http(state.clone(), http_addr).await;
    let grpc_handle = start_grpc(state.clone(), grpc_addr).await;

    // Wait for either the HTTP or gRPC server to exit
    let result = select(grpc_handle, http_handle).await;
    match result {
        Either::Left((grpc_result, http_handle)) => {
            http_handle.abort();

            if let Ok(Err(e)) = grpc_result {
                Err(e)
            } else {
                Err(BindError::Unknown)
            }
        }
        Either::Right((http_result, grpc_handle)) => {
            grpc_handle.abort();

            if let Ok(Err(e)) = http_result {
                Err(e)
            } else {
                Err(BindError::Unknown)
            }
        }
    }
}
