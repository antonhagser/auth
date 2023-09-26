use crypto::snowflake::Snowflake;
use tonic::Code;

use crate::{
    core::basic::register::{self, BasicRegistrationData, BasicRegistrationError},
    grpc::error::DetailedError,
    state::AppState,
};

/// Tonic-generated gRPC bindings
mod proto_basic {
    tonic::include_proto!("authcore.auth.basic");
}

pub use proto_basic::*;

use self::basic_auth_server::BasicAuth;

pub struct BasicServer {
    state: AppState,
}

impl BasicServer {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl BasicAuth for BasicServer {
    async fn register(
        &self,
        request: tonic::Request<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterResponse>, tonic::Status> {
        let data = request.into_inner();

        // Convert the application ID to a snowflake
        let application_id: Snowflake = if let Ok(id) = data.application_id.try_into() {
            id
        } else {
            return Err(tonic::Status::invalid_argument("application id is invalid"));
        };

        // Configure the registration data
        let data = BasicRegistrationData {
            email: data.email,

            first_name: None,
            last_name: None,

            password: data.password,
            application_id,
        };

        fn parse_err(e: BasicRegistrationError) -> tonic::Status {
            let error = match e {
                BasicRegistrationError::EmailFormat => (
                    ErrorCode::EmailFormat,
                    "invalid email format",
                    Code::InvalidArgument,
                ),
                BasicRegistrationError::PasswordFormat(_format) => {
                    // Todo: return format
                    (
                        ErrorCode::PasswordFormat,
                        "invalid password format",
                        Code::InvalidArgument,
                    )
                }
                BasicRegistrationError::AlreadyExists => (
                    ErrorCode::AlreadyExists,
                    "user already exists",
                    Code::AlreadyExists,
                ),
                BasicRegistrationError::ApplicationDoesNotExist => (
                    ErrorCode::ApplicationDoesNotExist,
                    "application does not exist",
                    Code::NotFound,
                ),
                BasicRegistrationError::InternalServerError => (
                    ErrorCode::InternalServerError,
                    "internal server error",
                    Code::Internal,
                ),
            };

            let register_error = DetailedError {
                code: error.0.to_string(),
                message: error.1.to_string(),
            };

            let out = serde_json::to_string(&register_error).unwrap();

            tonic::Status::new(error.2, out)
        }

        // Try to register the user
        let user = match register::with_basic_auth(&self.state, data).await {
            Err(e) => return Err(parse_err(e)),
            Ok(user) => user,
        };

        Ok(tonic::Response::new(RegisterResponse {
            user_id: user.id().to_string(),
        }))
    }
}
