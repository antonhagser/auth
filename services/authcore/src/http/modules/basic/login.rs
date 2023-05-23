use axum::{extract::State, Form, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    core::{
        basic::{login, login::BasicLoginData},
        token,
    },
    http::response::HTTPResponse,
    models::user::ExistsOr,
    state::AppState,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email_or_username: String,
    password: String,
    application_id: u64,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn route(
    State(state): State<AppState>,
    Form(data): Form<LoginRequest>,
) -> (StatusCode, Json<HTTPResponse>) {
    let email_or_username = if data.email_or_username.contains('@') {
        ExistsOr::Email(data.email_or_username)
    } else {
        ExistsOr::Username(data.email_or_username)
    };

    let login_data = BasicLoginData {
        email_or_username,
        password: data.password,
        application_id: data.application_id.try_into().unwrap(),
    };

    let user = match login::with_basic_auth(&state, login_data).await {
        Ok(user) => user,
        Err(_) => {
            let response =
                HTTPResponse::error("Unauthorized", "Invalid email or password".to_owned(), ());

            return (StatusCode::UNAUTHORIZED, Json(response));
        }
    };

    let (transaction, transaction_client) = state.prisma()._transaction().begin().await.unwrap();

    // Generate a new user refresh token
    let refresh_token = token::new_refresh_token(
        &transaction_client,
        state.id_generator(),
        user.id(),
        Utc::now() + Duration::days(30),
    )
    .await;

    let refresh_token = match refresh_token {
        Ok(refresh_token) => refresh_token,
        Err(e) => {
            error!("Failed to generate refresh token: {}", e);
            let _ = transaction.rollback(transaction_client).await;

            let response = HTTPResponse::error("InternalServerError", "A refresh token could not be created for the account due to an internal server error.", ());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // Generate access token
    let access_token = token::new_access_token(
        &state,
        user.id(),
        Utc::now() + Duration::hours(1),
        refresh_token.id(),
    );

    let access_token = match access_token {
        Ok(access_token) => access_token,
        Err(e) => {
            error!("Failed to generate access token: {}", e);
            let _ = transaction.rollback(transaction_client).await;

            let response = HTTPResponse::error("InternalServerError", "An access token could not be created for the account due to an internal server error.", ());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // Commit the transaction
    if transaction.commit(transaction_client).await.is_err() {
        let response = HTTPResponse::error("InternalServerError", "", ());
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }

    // Return the access token and refresh token to the client
    let response = LoginResponse {
        access_token,
        refresh_token: refresh_token.token().into(),
    };

    let response = HTTPResponse::ok(response);
    (StatusCode::OK, Json(response))
}
