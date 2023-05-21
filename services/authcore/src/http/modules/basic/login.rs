use axum::{extract::State, Form, Json};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    core::basic::{login, login::BasicLoginData},
    models::user::ExistsOr,
    state::AppState,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email_or_username: String,
    password: String,
    application_id: u64,
}

pub async fn route(
    State(state): State<AppState>,
    Form(data): Form<LoginRequest>,
) -> (StatusCode, Json<()>) {
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

    match login::with_basic_auth(&state, login_data).await {
        Ok(_) => {
            tracing::info!("login successful");
        }
        Err(e) => {
            tracing::error!("login error: {:#?}", e);
        }
    }

    (StatusCode::OK, Json(()))
}
