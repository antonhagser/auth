use hyper::StatusCode;
use serde::Serialize;

/// API error status codes
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorStatusCode {
    // Basic registration errors
    /// Invalid password
    InvalidPassword = 86500,
    /// Invalid email address
    InvalidEmailAddress = 86501,
    /// Email address already exists
    AlreadyExists = 86502,
    /// Invalid username
    InvalidUsername = 86503,

    /// Internal server error
    InternalServerError = 500,
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
            ErrorStatusCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
