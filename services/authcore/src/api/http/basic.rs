use axum::routing::post;
use tracing::trace;

use crate::state::State;

pub use login::login;

pub mod login;

pub fn router(state: State) -> axum::Router {
    trace!("registering basic auth routes");

    axum::Router::new()
        .with_state(state)
        .route("/login", post(login))
}
