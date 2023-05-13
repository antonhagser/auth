use axum::{extract::State, Form, Json};
use hyper::StatusCode;
use serde::Deserialize;
use tracing::info;

use crate::{
    models::{application::Application, user::User},
    state::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
}

pub async fn route(
    State(state): State<AppState>,
    Form(data): Form<RegisterRequest>,
) -> (StatusCode, Json<()>) {
    let t = std::time::Instant::now();
    let application = Application::builder(state.id_generator(), state.prisma())
        .name("Test Application".to_string())
        .build()
        .await
        .unwrap();
    let el = t.elapsed();
    info!("application: {:#?}", application);
    info!("elapsed: {:?}", el);

    // test user register
    let t = std::time::Instant::now();
    let user = User::builder(state.id_generator(), state.prisma(), application.id())
        .email_address(data.email)
        .build()
        .await
        .unwrap();
    let el = t.elapsed();

    info!("user: {:#?}", user);
    info!("elapsed: {:?}", el);

    (StatusCode::OK, Json(()))
}
