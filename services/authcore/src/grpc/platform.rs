use std::result::Result;

use tracing::error;

use crate::{
    models::application::{BasicAuthConfig, ReplicatedApplication},
    state::AppState,
};

use super::authcore::{
    AddApplicationRequest, AddApplicationResponse, DeleteApplicationRequest,
    DeleteApplicationResponse, GetAuthCoreVersionRequest, GetAuthCoreVersionResponse,
};

pub struct Platform {
    state: AppState,
}

impl Platform {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl super::authcore::auth_core_platform_server::AuthCorePlatform for Platform {
    async fn get_auth_core_version(
        &self,
        _request: tonic::Request<GetAuthCoreVersionRequest>,
    ) -> Result<tonic::Response<GetAuthCoreVersionResponse>, tonic::Status> {
        let response = GetAuthCoreVersionResponse {
            version: self.state.service_data().version.into(),
        };

        Ok(tonic::Response::new(response))
    }

    async fn add_application(
        &self,
        request: tonic::Request<AddApplicationRequest>,
    ) -> Result<tonic::Response<AddApplicationResponse>, tonic::Status> {
        let (_, _, data) = request.into_parts();

        let basic_auth_config_builder = BasicAuthConfig::builder();

        if let Err(e) = ReplicatedApplication::new_and_insert(
            self.state.prisma(),
            data.application_id.try_into().unwrap(),
            basic_auth_config_builder,
        )
        .await
        {
            // Check if prisma error is: already exists
            if e.is_prisma_error::<prisma_client_rust::prisma_errors::query_engine::UniqueKeyViolation>()
            {
                return Err(tonic::Status::already_exists("application already exists"));
            }

            error!("failed to create application: {}", e);
            return Err(tonic::Status::internal("internal server error"));
        }

        let response = AddApplicationResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn delete_application(
        &self,
        request: tonic::Request<DeleteApplicationRequest>,
    ) -> Result<tonic::Response<DeleteApplicationResponse>, tonic::Status> {
        let (_, _, data) = request.into_parts();

        let application_id = data.application_id.try_into().unwrap();
        ReplicatedApplication::delete(self.state.prisma(), application_id)
            .await
            .map_err(|_| tonic::Status::internal("internal server error"))?;

        Ok(tonic::Response::new(DeleteApplicationResponse {}))
    }
}
