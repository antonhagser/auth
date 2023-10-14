use axum::routing::post;
use tracing::trace;

use crate::state::State;

mod refresh;

pub fn router(state: State) -> axum::Router {
    trace!("registering token routes");

    axum::Router::new()
        .with_state(state)
        .route("/refresh", post(refresh::refresh))
}
