use axum::{extract::State, Form, Json};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    core::basic::register::{self, BasicRegistrationData},
    http::response::HTTPResponse,
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
                let error = HTTPResponse::error(
                    "InvalidEmailAddress",
                    "Invalid email address".to_owned(),
                    (),
                );

                return (StatusCode::BAD_REQUEST, Json(error));
            }
            register::BasicRegistrationError::EmailAddressAlreadyExists => {
                let error = HTTPResponse::error(
                    "AlreadyExists",
                    "Email address already in use".to_owned(),
                    (),
                );

                return (StatusCode::CONFLICT, Json(error));
            }
            register::BasicRegistrationError::InvalidPassword(reason) => {
                let error =
                    HTTPResponse::error("InvalidPassword", "Invalid password".to_owned(), reason);

                return (StatusCode::BAD_REQUEST, Json(error));
            }
            register::BasicRegistrationError::InvalidUsernameFormat => {
                let error =
                    HTTPResponse::error("InvalidUsername", "Invalid username".to_owned(), ());

                return (StatusCode::BAD_REQUEST, Json(error));
            }
            register::BasicRegistrationError::UsernameAlreadyExists => {
                let error =
                    HTTPResponse::error("AlreadyExists", "Username already in use".to_owned(), ());

                return (StatusCode::CONFLICT, Json(error));
            }
            register::BasicRegistrationError::QueryError(_) => {
                let error = HTTPResponse::error(
                    "InternalServerError",
                    "Internal server error".to_owned(),
                    (),
                );

                return (StatusCode::INTERNAL_SERVER_ERROR, Json(error));
            }
            register::BasicRegistrationError::Unknown => {
                let error = HTTPResponse::error(
                    "InternalServerError",
                    "Internal server error".to_owned(),
                    (),
                );

                return (StatusCode::INTERNAL_SERVER_ERROR, Json(error));
            }
            register::BasicRegistrationError::ApplicationDoesNotExist => {
                let error = HTTPResponse::error(
                    "ApplicationDoesNotExist",
                    "Application does not exist".to_owned(),
                    (),
                );

                return (StatusCode::BAD_REQUEST, Json(error));
            }
        }
    };

    (StatusCode::OK, Json(HTTPResponse::empty()))
}
