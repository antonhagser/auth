use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Serialize)]
pub enum UsernameValidationError {
    #[error("invalid length")]
    InvalidLength,

    #[error("invalid characters")]
    InvalidCharacters,
}

pub fn validate_username(username: &str) -> Result<(), Vec<UsernameValidationError>> {
    // Must only contain alphanumeric characters or single hyphens, and cannot begin or end with a hyphen.
    // Must be between 1 and 63 characters long.

    if username.is_empty() || username.len() > 63 {
        return Err(vec![UsernameValidationError::InvalidLength]);
    }

    let mut chars = username.chars().peekable();
    let mut prev_char = None;
    while let Some(c) = chars.next() {
        if !c.is_ascii_alphanumeric() && c != '-' {
            return Err(vec![UsernameValidationError::InvalidCharacters]);
        }
        if c == '-'
            && (prev_char.is_none()
                || chars.peek().is_none()
                || !chars.peek().unwrap().is_ascii_alphanumeric())
        {
            return Err(vec![UsernameValidationError::InvalidCharacters]);
        }
        prev_char = Some(c);
    }

    Ok(())
}
