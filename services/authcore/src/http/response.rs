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
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
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
    pub fn error<D, C, M>(code: C, message: M, details: D) -> Self
    where
        C: Into<String>,
        M: Into<String>,
        D: Serialize,
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
