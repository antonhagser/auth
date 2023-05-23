use serde::Serialize;

/// Common HTTP response struct
///
/// # Structure
/// ```json
/// {
///     "success": true,
///     "data": {...},
///     "error": {
///       "code": 404,
///       "message": "Resource not found",
///       "details": {...}
///     }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HTTPResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<HTTPResponseError>,
}

/// HTTP response error struct
///
/// # Structure
/// ```json
/// {
///    "code": 404,
///    "message": "Resource not found",
///    "details": {...}
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct HTTPResponseError {
    code: String,
    message: String,
    details: serde_json::Value,
}

impl HTTPResponse {
    /// Create a new HTTP response with data
    pub fn ok<T>(data: T) -> Self
    where
        T: Serialize,
    {
        let data = serde_json::to_value(data).unwrap();

        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create a new HTTP response with error
    pub fn error<T, E, M>(code: E, message: M, details: T) -> Self
    where
        T: Serialize,
        E: Into<String>,
        M: Into<String>,
    {
        let details = serde_json::to_value(details).unwrap();

        Self {
            success: false,
            data: None,
            error: Some(HTTPResponseError {
                code: code.into(),
                message: message.into(),
                details,
            }),
        }
    }

    // Empty
    pub fn empty() -> Self {
        Self {
            success: true,
            data: None,
            error: None,
        }
    }

    /// Return in JSON format
    pub fn json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    /// Return in Axum JSON format
    pub fn axum_json(self) -> axum::Json<Self> {
        axum::Json(self)
    }
}
