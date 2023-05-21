use hyper::StatusCode;
use serde::Serialize;

/// API error status codes
#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u32)]
pub enum ErrorStatusCode {
    // Basic registration errors
    /// Invalid password
    InvalidPassword,
    /// Invalid email address
    InvalidEmailAddress,
    /// Email address already exists
    AlreadyExists,
    /// Invalid username
    InvalidUsername,

    /// Application does not exist
    ApplicationDoesNotExist,

    /// Internal server error
    InternalServerError,
}

impl ErrorStatusCode {
    /// Get the HTTP status code of the error
    pub fn http_status_code(&self) -> StatusCode {
        #[allow(unreachable_patterns)]
        match self {
            ErrorStatusCode::InvalidPassword => StatusCode::BAD_REQUEST,
            ErrorStatusCode::InvalidEmailAddress => StatusCode::BAD_REQUEST,
            ErrorStatusCode::AlreadyExists => StatusCode::CONFLICT,
            ErrorStatusCode::InvalidUsername => StatusCode::BAD_REQUEST,
            ErrorStatusCode::ApplicationDoesNotExist => StatusCode::BAD_REQUEST,
            ErrorStatusCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
