use std::sync::Arc;

#[derive(Debug)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub type AppState = Arc<State>;
