use crate::state::AppState;

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
        _request: tonic::Request<super::authcore::GetAuthCoreVersionRequest>,
    ) -> std::result::Result<
        tonic::Response<super::authcore::GetAuthCoreVersionResponse>,
        tonic::Status,
    > {
        let response = super::authcore::GetAuthCoreVersionResponse {
            version: self.state.service_data().version.into(),
        };

        Ok(tonic::Response::new(response))
    }

    async fn delete_application(
        &self,
        _request: tonic::Request<super::authcore::DeleteApplicationRequest>,
    ) -> std::result::Result<
        tonic::Response<super::authcore::DeleteApplicationResponse>,
        tonic::Status,
    > {
        Err(tonic::Status::unimplemented("not implemented"))
    }

    async fn get_application(
        &self,
        _request: tonic::Request<super::authcore::GetApplicationRequest>,
    ) -> std::result::Result<tonic::Response<super::authcore::GetApplicationResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not implemented"))
    }

    async fn add_application(
        &self,
        _request: tonic::Request<super::authcore::AddApplicationRequest>,
    ) -> std::result::Result<tonic::Response<super::authcore::AddApplicationResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not implemented"))
    }
}
