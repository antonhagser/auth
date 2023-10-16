use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod basic_auth;
mod email_address;

pub use basic_auth::*;
pub use email_address::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub(crate) user_id: Uuid,
    pub(crate) application_id: Uuid,
    pub(crate) external_id: String,
    pub(crate) primary_email_id: Option<Uuid>,
    pub(crate) full_name: Option<String>,
    pub(crate) display_name: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub(crate) user_id: Uuid,
    pub(crate) application_id: Uuid,
    pub(crate) external_id: String,
    pub(crate) primary_email_id: Option<Uuid>,
    pub(crate) full_name: Option<String>,
    pub(crate) display_name: String,
}
