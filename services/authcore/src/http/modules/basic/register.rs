use axum::{
    extract::{FromRequest, State},
    Form, Json,
};
use crypto::snowflake::Snowflake;
use hyper::{Body, Request, StatusCode};
use serde::Deserialize;

use crate::{
    core::basic::register::{self, BasicRegistrationData},
    http::response::HTTPResponse,
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    application_id: String,
}

async fn get_register_request(request: Request<Body>) -> Option<RegisterRequest> {
    match request
        .headers()
        .get("content-type")
        .and_then(|header| header.to_str().ok())
    {
        Some("application/x-www-form-urlencoded") => {
            let data = Form::<RegisterRequest>::from_request(request, &())
                .await
                .ok()?;

            Some(data.0)
        }
        Some("application/json") => {
            let data = Json::<RegisterRequest>::from_request(request, &())
                .await
                .ok()?;

            Some(data.0)
        }
        _ => None,
    }
}

pub async fn route(
    State(state): State<AppState>,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
    // Accept multiple different ways to give the data, url encoded form data or json body
    let data: RegisterRequest = match get_register_request(request).await {
        Some(d) => d,
        None => {
            let response = HTTPResponse::error(
                    "BadRequest",
                    "Invalid content type, expected application/x-www-form-urlencoded or application/json".to_owned(),
                    (),
                );

            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    // Convert the application ID to a snowflake
    let application_id: Snowflake = if let Ok(id) = data.application_id.try_into() {
        id
    } else {
        let error = HTTPResponse::error(
            "InvalidApplicationID",
            "Invalid application ID".to_owned(),
            (),
        );

        return (StatusCode::BAD_REQUEST, Json(error));
    };

    // Configure the registration data
    let data = BasicRegistrationData {
        email: data.email,

        first_name: None,
        last_name: None,

        password: data.password,
        application_id,
    };

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
