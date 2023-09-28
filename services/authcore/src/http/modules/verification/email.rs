use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::{Body, HeaderMap, Request};
use serde::Serialize;
use tracing::error;

use crate::{
    core::verification::email::EmailVerificationToken,
    http::response::HTTPResponse,
    models::{
        prisma::UserTokenType,
        user::{EmailAddress, UserToken},
    },
    state::AppState,
};

#[derive(strum::Display)]
enum Error {
    InvalidToken,
    EmailNotFound,
    #[allow(clippy::enum_variant_names)]
    InternalServerError,
}

// TODDO: Make into trait for easier error handling (insert cool emojo 😎)
impl Error {
    pub fn get_message<'a>(&self) -> &'a str {
        match self {
            Error::InvalidToken => "an invalid token provided",
            Error::EmailNotFound => "the email to verify was not found",
            Error::InternalServerError => "and internal server error occured",
        }
    }
}

#[derive(Serialize)]
struct Response {}

pub async fn route(
    Path(token): Path<String>,
    State(state): State<AppState>,
    _request: Request<Body>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let token = EmailVerificationToken::from_raw(&state, &token);
    let token = match token {
        Err(e) => {
            error!("Invalid token when parsing: {}", e);
            return (
                hyper::StatusCode::UNAUTHORIZED,
                headers,
                Json(HTTPResponse::error(
                    Error::InvalidToken.to_string(),
                    Error::InvalidToken.get_message(),
                    (),
                )),
            );
        }
        Ok(t) => t,
    };

    // Extract data from token
    let token_data = token.data;
    let token_id = token.token_id;
    let user_id = token_data.user_id;
    let email_id = token_data.email_id;
    let application_id = token_data.application_id;

    tracing::debug!("Email verification token parsed successfully");
    tracing::debug!("token_id: {}", token_id);
    tracing::debug!("user_id: {}", user_id);
    tracing::debug!("email_id: {}", email_id);
    tracing::debug!("application_id: {}", application_id);

    let token = UserToken::get(
        state.prisma(),
        user_id,
        token_id,
        UserTokenType::EmailVerification,
    )
    .await;
    let token = match token {
        Ok(token) => token,
        Err(e) => {
            error!("Invalid token: {}", e);
            return (
                hyper::StatusCode::UNAUTHORIZED,
                headers,
                Json(HTTPResponse::error(
                    Error::InvalidToken.to_string(),
                    Error::InvalidToken.get_message(),
                    (),
                )),
            );
        }
    };

    let email_address =
        match EmailAddress::get(state.prisma(), token.user_id(), email_id, application_id).await {
            Ok(user) => user,
            Err(_) => {
                return (
                    hyper::StatusCode::UNAUTHORIZED,
                    headers,
                    Json(HTTPResponse::error(
                        Error::EmailNotFound.to_string(),
                        Error::EmailNotFound.get_message(),
                        (),
                    )),
                )
            }
        };

    // Update email address
    let result = email_address
        .set_verified(state.prisma(), application_id)
        .await;
    if result.is_err() {
        return (
            hyper::StatusCode::UNAUTHORIZED,
            headers,
            Json(HTTPResponse::error(
                Error::InternalServerError.to_string(),
                Error::InternalServerError.get_message(),
                (),
            )),
        );
    }

    // Header for redirect
    // TODO: Get redirect URL from token
    headers.insert(
        "Location",
        "https://example.com/verification/email/success"
            .parse()
            .unwrap(),
    );

    let response = HTTPResponse::ok(Response {});
    (hyper::StatusCode::FOUND, headers, Json(response))
}
