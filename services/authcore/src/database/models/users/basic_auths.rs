use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAuth {
    pub(crate) user_id: Uuid,
    pub(crate) application_id: Uuid,
    pub(crate) password_hash: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NewBasicAuth {
    pub(crate) user_id: Uuid,
    pub(crate) application_id: Uuid,
    pub(crate) password_hash: String,
}
