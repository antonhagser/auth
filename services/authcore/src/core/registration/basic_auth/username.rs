use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Serialize)]
pub enum UsernameValidationError {}

pub fn validate_username(_username: &str) -> Result<(), Vec<UsernameValidationError>> {
    // TODO: validate username
    Ok(())
}
