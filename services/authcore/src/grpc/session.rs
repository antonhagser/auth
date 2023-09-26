/// Tonic-generated gRPC bindings
mod proto_session {
    tonic::include_proto!("authcore.session");
}

pub use proto_session::*;
use tracing::error;

use crate::{core, state::AppState};

use self::session_server::Session;

pub struct SessionServer {
    state: AppState,
}

impl SessionServer {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Session for SessionServer {
    async fn validate(
        &self,
        request: tonic::Request<ValidateRequest>,
    ) -> Result<tonic::Response<ValidateResponse>, tonic::Status> {
        let data = request.into_inner();

        let result = core::token::verify_access_token(&self.state, &data.refresh_token);
        if let Err(e) = result {
            error!("Failed to validate access token: {:?}", e);
            return Err(tonic::Status::unauthenticated("Invalid access token"));
        }

        Ok(tonic::Response::new(ValidateResponse {}))
    }

    async fn invalidate(
        &self,
        _request: tonic::Request<InvalidateRequest>,
    ) -> Result<tonic::Response<InvalidateResponse>, tonic::Status> {
        unimplemented!()
    }
}
