use std::{net::{Ipv4Addr, SocketAddr}, io::Write};

use axum::{routing::get, Json, Router};
use once_cell::sync::Lazy;
use serde::Serialize;
use tracing::info;
use utoipa::{OpenApi, ToSchema};

#[derive(Clone, Copy, Serialize, ToSchema)]
struct ServiceData {
    service_name: &'static str,
    service_version: &'static str,
}

static SERVICE_DATA: Lazy<ServiceData> = Lazy::new(|| ServiceData {
    service_name: "authentication",
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("starting service");

    let app = Router::new().route("/", get(root));

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

    let addr = SocketAddr::from((ip, 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
