use axum::routing::post;

use crate::state::State;

pub use login::login;
pub use register::register;

pub mod login;
pub mod register;

pub fn router(state: State) -> axum::Router {
    axum::Router::new()
        .with_state(state)
        .route("/", post(register))
        .route("/login", post(login))
}
