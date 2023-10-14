use axum::routing::post;
use tracing::trace;

use crate::state::State;

use self::{
    create::create_multi_factor, disable::disable_multi_factor, verify::verify_multi_factor,
};

pub mod create;
pub mod disable;
pub mod verify;

pub fn router(state: State) -> axum::Router {
    trace!("registering mfa routes");

    axum::Router::new()
        .with_state(state)
        .route("/", post(create_multi_factor))
        .route("/verify", post(verify_multi_factor))
        .route("/disable", post(disable_multi_factor))
}
