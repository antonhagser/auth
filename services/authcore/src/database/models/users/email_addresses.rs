use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub(crate) email_id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) application_id: Uuid,
    pub(crate) email: String,
    pub(crate) is_primary: bool,
    pub(crate) is_verified: bool,
    pub(crate) verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NewEmailAddress {
    pub(crate) email_id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) application_id: Uuid,
    pub(crate) email: String,
    pub(crate) is_primary: bool,
    pub(crate) is_verified: bool,
    pub(crate) verified_at: Option<chrono::DateTime<chrono::Utc>>,
}
