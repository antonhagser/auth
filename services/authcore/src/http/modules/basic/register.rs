use axum::{extract::State, Form, Json};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    core::basic::register::{self, BasicRegistrationData},
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
    let data = BasicRegistrationData {
        email: data.email,
        username: data.username,

        first_name: None,
        last_name: None,

        password: data.password,
        application_id: data.application_id.try_into().unwrap(),
    };

    // TODO: VERY IMPORTANT
    // ! Must check with platform service that the organization and application exists -
    // ! otherwise the user will be created without an organization and application WHICH IS BAD

    // Try to register the user
    if let Err(e) = register::with_basic_auth(&state, data).await {
        tracing::error!("registration error: {:#?}", e);

        match e {
            register::BasicRegistrationError::InvalidEmailAddressFormat => {
                let code = ErrorStatusCode::InvalidEmailAddress;
                let error = HTTPResponse::error(code, "Invalid email address".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::EmailAddressAlreadyExists => {
                let code = ErrorStatusCode::AlreadyExists;
                let error =
                    HTTPResponse::error(code, "Email address already in use".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::InvalidPassword(reason) => {
                let code = ErrorStatusCode::InvalidPassword;
                let error = HTTPResponse::error(code, "Invalid password".to_owned(), reason);

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::InvalidUsernameFormat => {
                let code = ErrorStatusCode::InvalidUsername;
                let error = HTTPResponse::error(code, "Invalid username".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::UsernameAlreadyExists => {
                let code = ErrorStatusCode::AlreadyExists;
                let error = HTTPResponse::error(code, "Username already in use".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::QueryError(_) => {
                let code = ErrorStatusCode::InternalServerError;
                let error = HTTPResponse::error(code, "Internal server error".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::Unknown => {
                let code = ErrorStatusCode::InternalServerError;
                let error = HTTPResponse::error(code, "Internal server error".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
            register::BasicRegistrationError::ApplicationDoesNotExist => {
                let code = ErrorStatusCode::ApplicationDoesNotExist;
                let error = HTTPResponse::error(code, "Application does not exist".to_owned(), ());

                return (code.http_status_code(), Json(error));
            }
        }
    };

    (StatusCode::OK, Json(HTTPResponse::empty()))
}
