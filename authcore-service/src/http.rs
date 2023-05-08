use std::{io::Write, net::SocketAddr};

use axum::{routing::get, Json, Router};
use utoipa::OpenApi;

use crate::{state::AppState, ServiceData, SERVICE_DATA};

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

#[derive(Debug, thiserror::Error)]
pub enum HTTPServerError {
    #[error("failed to build OpenAPI documentation")]
    OpenAPIIOError(std::io::Error),

    #[error("critical error in Axum (HTTPServer)")]
    ServerError(hyper::Error),
}

fn build_openapi() -> Result<(), HTTPServerError> {
    // Build the OpenAPI documentation
    #[derive(OpenApi)]
    #[openapi(paths(root), components(schemas(ServiceData)))]
    struct ApiDoc;

    let doc = ApiDoc::openapi()
        .to_pretty_json()
        .expect("failed to serialize OpenAPI document");

    std::fs::File::create("api.json")
        .map_err(HTTPServerError::OpenAPIIOError)?
        .write_all(doc.as_bytes())
        .map_err(HTTPServerError::OpenAPIIOError)?;

    Ok(())
}

pub async fn run(addr: SocketAddr, state: AppState) -> Result<(), HTTPServerError> {
    #[cfg(debug_assertions)]
    {
        build_openapi()?;
    }

    let app = Router::new()
        .route("/", get(root))
        .with_state(state.clone());

    tracing::debug!("http listening on {}", addr);

    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Err(e) => Err(HTTPServerError::ServerError(e)),
        Ok(_) => Ok(()),
    }
}
