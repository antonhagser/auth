use std::result::Result;

use axum::async_trait;
use tonic::{Request, Response};

use crate::state::State;

use self::authcore::{
    AddApplicationRequest, AddApplicationResponse, DeleteApplicationRequest,
    DeleteApplicationResponse,
};

/// Tonic-generated gRPC bindings
pub mod authcore {
    tonic::include_proto!("authcore");
}

#[derive(Clone)]
pub struct PlatformService {
    pub state: State,
}

#[async_trait]
impl authcore::platform_server::Platform for PlatformService {
    async fn add_application(
        &self,
        _request: Request<AddApplicationRequest>,
    ) -> Result<Response<AddApplicationResponse>, tonic::Status> {
        unimplemented!("add_application")
    }

    async fn delete_application(
        &self,
        _request: Request<DeleteApplicationRequest>,
    ) -> Result<Response<DeleteApplicationResponse>, tonic::Status> {
        unimplemented!("delete_application")
    }
}
