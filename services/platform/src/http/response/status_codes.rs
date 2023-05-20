use hyper::StatusCode;
use serde::Serialize;

/// API error status codes
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorStatusCode {
    /// Internal server error
    InternalServerError = 500,
}

impl ErrorStatusCode {
    /// Get the HTTP status code of the error
    pub fn http_status_code(&self) -> StatusCode {
        #[allow(unreachable_patterns)]
        match self {
            ErrorStatusCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
