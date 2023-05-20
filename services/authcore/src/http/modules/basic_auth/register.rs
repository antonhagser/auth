use axum::{extract::State, Form, Json};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    core::registration,
    http::response::{ErrorStatusCode, HTTPResponse},
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: Option<String>,
    password: String,
    application_id: u64,
}

pub async fn route(
    State(state): State<AppState>,
    Form(data): Form<RegisterRequest>,
) -> (StatusCode, Json<HTTPResponse>) {
    // Configure the registration data
    let data = registration::BasicRegistrationData {
        email: data.email,
        username: data.username,
        password: data.password,
        application_id: data.application_id,
    };

    // TODO: VERY IMPORTANT
    // ! Must check with platform service that the organization and application exists -
    // ! otherwise the user will be created without an organization and application WHICH IS BAD

    // Try to register the user
    if let Err(e) = registration::with_basic_auth(&state, data).await {
        tracing::error!("registration error: {:#?}", e);

        match e {
            registration::BasicRegistrationError::InvalidEmailAddress => {
                let code = ErrorStatusCode::InvalidEmailAddress;
                let error = HTTPResponse::error(code, "Invalid email address".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            registration::BasicRegistrationError::EmailAddressAlreadyExists => {
                let code = ErrorStatusCode::AlreadyExists;
                let error =
                    HTTPResponse::error(code, "Email address already in use".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            registration::BasicRegistrationError::InvalidPassword(reason) => {
                let code = ErrorStatusCode::InvalidPassword;
                let error = HTTPResponse::error(code, "Invalid password".to_owned(), reason);

                return (code.http_status_code(), Json(error));
            }
            registration::BasicRegistrationError::InvalidUsername => {
                let code = ErrorStatusCode::InvalidUsername;
                let error = HTTPResponse::error(code, "Invalid username".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            registration::BasicRegistrationError::UsernameAlreadyExists => {
                let code = ErrorStatusCode::AlreadyExists;
                let error = HTTPResponse::error(code, "Username already in use".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            registration::BasicRegistrationError::QueryError(_) => {
                let code = ErrorStatusCode::InternalServerError;
                let error = HTTPResponse::error(code, "Internal server error".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
        }
    };

    (StatusCode::OK, Json(HTTPResponse::empty()))
}
