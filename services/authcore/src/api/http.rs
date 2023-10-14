use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router};

use crate::state::State;

use super::BindError;

mod basic;
mod mfa;
mod service;
mod token;

/// The root endpoint for the HTTP server. Used for health checks.
async fn root() -> impl IntoResponse {
    (hyper::StatusCode::IM_A_TEAPOT, "I'm a teapot")
}

#[derive(Debug, thiserror::Error)]
pub enum HTTPServerError {
    #[error("critical error in Axum (HTTPServer)")]
    Server(hyper::Error),
}

/// Starts and runs the HTTP server on the given address with the provided state.
///
/// # Arguments
///
/// * `addr`: The socket address to bind the server to.
/// * `state`: The shared application state to use across all routes.
pub async fn run(addr: SocketAddr, state: State) -> Result<(), BindError> {
    // Create the router
    let app: Router = Router::new()
        .with_state(state.clone())
        .route("/", get(root))
        .route("/health", get(service::health))
        .route("/ready", get(service::ready))
        .route("/version", get(service::version))
        .nest("/basic", basic::router(state.clone()))
        .nest("/token", token::router(state.clone()))
        .nest("/mfa", mfa::router(state.clone()));

    tracing::info!("http listening on {}", addr);

    // Start the server
    match axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
    {
        Err(e) => Err(BindError::HTTP(HTTPServerError::Server(e))),
        Ok(_) => Ok(()),
    }
}
