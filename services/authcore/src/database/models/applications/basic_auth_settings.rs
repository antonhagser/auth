use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAuthSettings {
    pub(crate) application_id: Uuid,
    pub(crate) min_password_length: i32,
    pub(crate) max_password_length: i32,
    pub(crate) require_lowercase: bool,
    pub(crate) require_uppercase: bool,
    pub(crate) require_numeric: bool,
    pub(crate) require_special: bool,
    pub(crate) password_history_count: i32,
    pub(crate) password_expiry_days: i32,
    pub(crate) max_failed_attempts: i32,
    pub(crate) lockout_duration: i32,
    pub(crate) require_mfa: bool,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}
