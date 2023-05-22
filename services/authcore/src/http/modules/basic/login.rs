use axum::{extract::State, Form, Json};
use chrono::{Duration, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    core::basic::{login, login::BasicLoginData},
    http::response::HTTPResponse,
    models::{
        prisma::UserTokenType,
        user::{ExistsOr, UserToken},
    },
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

    // Generate a new user refresh token
    let user_token = UserToken::builder(state.id_generator())
        .user_id(user.id())
        .token_type(UserTokenType::Refresh)
        .token("".into())
        .expires_at(Utc::now() + Duration::days(30))
        .build(state.prisma())
        .await;

    let user_token = match user_token {
        Ok(token) => token,
        Err(_) => {
            let error = HTTPResponse::error(
                "InternalServerError",
                "Internal server error".to_owned(),
                (),
            );

            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error));
        }
    };

    // TODO: Generate a new user access token
    let response = LoginResponse {
        access_token: "UNIMPLEMENTED".into(),
        refresh_token: user_token.token().into(),
    };

    let response = HTTPResponse::ok(response);

    (StatusCode::OK, Json(response))
}
