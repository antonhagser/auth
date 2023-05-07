use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use authenticator::Authenticator;
use axum::{routing::get, Json, Router};

use once_cell::sync::Lazy;
use serde::Serialize;
use tracing::info;
use utoipa::{OpenApi, ToSchema};

pub mod authenticator;
pub mod grpc;
pub mod routes;

#[derive(Clone, Copy, Serialize, ToSchema)]
struct ServiceData {
    service_name: &'static str,
    service_version: &'static str,
}

static SERVICE_DATA: Lazy<ServiceData> = Lazy::new(|| ServiceData {
    service_name: "AuthCore",
    service_version: env!("CARGO_PKG_VERSION"),
});

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "service information", body = ServiceData)
    )
)]
async fn root() -> Json<ServiceData> {
    Json(*SERVICE_DATA)
}

#[derive(Debug)]
pub struct State {
    authenticator: Authenticator,
}

pub type AppState = Arc<State>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("starting service");

    // Create a shared connection pool
    let shared_state = State {
        authenticator: Authenticator::new(),
    };

    let shared_state = Arc::new(shared_state);

    let ip = if cfg!(debug_assertions) {
        tracing::debug!("running in debug mode");

        // Build the OpenAPI documentation
        #[derive(OpenApi)]
        #[openapi(paths(root), components(schemas(ServiceData)))]
        struct ApiDoc;

        let doc = ApiDoc::openapi().to_pretty_json().unwrap();
        std::fs::File::create("api.json")
            .unwrap()
            .write_all(doc.as_bytes())
            .unwrap();

        // In debug mode we bind to localhost to avoid exposing the service
        Ipv4Addr::LOCALHOST
    } else {
        tracing::debug!("running in release mode");
        Ipv4Addr::UNSPECIFIED
    };

    let http_addr = SocketAddr::from((ip, 8080));
    let grpc_addr = SocketAddr::from((ip, 58080));

    let state = shared_state.clone();
    tokio::spawn(async move {
        grpc::run(grpc_addr, state).await;
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/verify", get(routes::verify::verify))
        .with_state(shared_state.clone());

    tracing::debug!("http listening on {}", http_addr);
    axum::Server::bind(&http_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    info!("service stopped");
}
