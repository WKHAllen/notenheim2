use backend_common::backend_commands;
use commands::BackendCommands;

pub struct AppState;

impl AppState {
    pub fn new() -> Self {
        Self
    }
}

#[backend_commands]
impl BackendCommands for AppState {
    async fn add(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}
