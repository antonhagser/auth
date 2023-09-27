use crypto::snowflake::Snowflake;
use tonic::Code;

use crate::{
    core::{
        basic::register::{self, BasicRegistrationData, BasicRegistrationError},
        token::generate_generic_token,
    },
    grpc::{
        client::email::{
            send_verification_email_request::Verification, EmailApplication, EmailData,
            SendVerificationEmailRequest,
        },
        error::DetailedError,
    },
    models::{prisma::UserTokenType, user::UserToken},
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

        let (transaction, prisma_client) = self
            .state
            .prisma()
            ._transaction()
            .begin()
            .await
            .map_err(|_| tonic::Status::internal("failed to create transaction"))?;

        // Try to register the user
        let user = match register::with_basic_auth(
            &self.state,
            &prisma_client,
            BasicRegistrationData {
                email: data.email.clone(),

                password: data.password,
                application_id,
            },
        )
        .await
        {
            Err(e) => return Err(parse_err(e)),
            Ok(user) => user,
        };

        // Send verification email
        let method = VerificationMethod::from_i32(data.verification_method).ok_or(
            tonic::Status::invalid_argument("verification method is invalid"),
        )?;

        // Generate a verification token
        let id_generator = self.state.id_generator();

        // If method is email generate a token
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .ok_or(tonic::Status::internal("failed to add duration"))?;

        // TODO: Build according to clean code principle
        // TODO: Move logic into sub function to handle errors properly

        let token = match method {
            VerificationMethod::EmailLink => generate_generic_token(
                &self.state,
                id_generator.next_snowflake().unwrap(),
                user.id(),
                expires_at,
            )
            .map_err(|_| tonic::Status::internal("failed to generate token"))?,
            VerificationMethod::EmailCode => unimplemented!(),
        };

        let user_token = UserToken::builder(
            id_generator.next_snowflake().unwrap(),
            user.id(),
            UserTokenType::EmailVerification,
            token,
            expires_at,
        )
        .build(&prisma_client)
        .await
        .map_err(|_| tonic::Status::internal("failed to create user token"))?;

        // Send email with gRPC
        let request = tonic::Request::new(SendVerificationEmailRequest {
            email_data: Some(EmailData {
                from: "verify@antonhagser.se".into(),
                to: vec![data.email],
                cc: vec![],
                bcc: vec![],
                reply_to: "".into(),
            }),
            email_application: Some(EmailApplication {
                name: "antonhagser.se".into(),
            }),
            verification: Some(Verification::VerificationUrl(format!(
                "http://localhost:/verify-email?token={}",
                user_token.token()
            ))),
        });

        // TODO: Handle error properly
        let mut email_grpc_client = self.state.email_grpc_client().lock().await;
        email_grpc_client.send_verification_email(request).await?;
        drop(email_grpc_client);

        // Commit the transaction
        transaction
            .commit(prisma_client)
            .await
            .map_err(|_| tonic::Status::internal("failed to commit transaction"))?;

        Ok(tonic::Response::new(RegisterResponse {
            user_id: user.id().to_string(),
        }))
    }
}
