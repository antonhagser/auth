use axum::routing::post;

use crate::state::State;

use self::{create::create_two_factor, verify::verify_two_factor};

pub mod create;
pub mod verify;

pub fn router(state: State) -> axum::Router {
    axum::Router::new()
        .with_state(state)
        .route("/", post(create_two_factor))
        .route("/verify", post(verify_two_factor))
}
