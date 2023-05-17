use axum::{extract::State, Form, Json};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{core::registration, state::AppState};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
}

pub async fn route(
    State(state): State<AppState>,
    Form(data): Form<RegisterRequest>,
) -> (StatusCode, Json<()>) {
    let email = data.email;

    let data = registration::RegistrationData {
        email,
        username: "".to_owned(),
        password: "".to_owned(),
        phone_number: "".to_owned(),
        application_id: 0,
    };

    let _ = registration::with_basic_auth(&state, data).await;

    (StatusCode::OK, Json(()))
}
