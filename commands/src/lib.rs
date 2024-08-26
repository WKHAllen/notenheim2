use macros::command_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandError {
    InvalidCommand(String),
    MalformedRequest(String),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::InvalidCommand(cmd) => format!("invalid command: {}", cmd),
            Self::MalformedRequest(cmd) => format!("malformed command: {}", cmd),
        })
    }
}

impl std::error::Error for CommandError {}

#[derive(Serialize, Deserialize)]
pub struct CommandRequest {
    pub name: String,
    pub req: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommandResponse {
    pub res: Result<String, CommandError>,
}

/// Global application commands, designed to facilitate communication between
/// the frontend and backend.
#[command_trait]
pub trait Commands {
    /// Adds two numbers together.
    async fn add(&self, x: i32, y: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
