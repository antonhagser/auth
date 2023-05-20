//! This module defines an HTTP server using the Axum framework
//! and handles OpenAPI documentation generation.

use std::net::SocketAddr;

use axum::{routing::get, Json, Router};

use crate::{state::AppState, ServiceData, SERVICE_DATA};

pub mod modules;
pub mod response;

async fn root() -> Json<ServiceData> {
    Json(*SERVICE_DATA)
}

/// HTTPServerError enumerates the possible errors that can occur
/// when running the HTTP server or building OpenAPI documentation.
///
/// Possible errors include:
/// - `OpenAPIIOError`: An error occurred while building the OpenAPI documentation.
/// - `ServerError`: An error occurred within the Axum (HTTPServer).
#[derive(Debug, thiserror::Error)]
pub enum HTTPServerError {
    #[error("failed to build OpenAPI documentation")]
    OpenAPIIOError(std::io::Error),

    #[error("critical error in Axum (HTTPServer)")]
    ServerError(hyper::Error),
}

/// Starts and runs the HTTP server on the given address with the provided state.
///
/// # Arguments
///
/// * `addr`: The socket address to bind the server to.
/// * `state`: The shared application state to use across all routes.
///
/// # Errors
///
/// Returns an `HTTPServerError::ServerError` if there is an issue
/// within the Axum (HTTPServer).
pub async fn run(addr: SocketAddr, state: AppState) -> Result<(), HTTPServerError> {
    let app = Router::new()
        .route("/platform", get(root))
        .with_state(state.clone())
        .nest("/platform", modules::router(state.clone()));

    tracing::debug!("http listening on {}", addr);

    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Err(e) => Err(HTTPServerError::ServerError(e)),
        Ok(_) => Ok(()),
    }
}
