use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod basic_auth_settings;

pub use basic_auth_settings::BasicAuthSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub(crate) application_id: Uuid,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}
