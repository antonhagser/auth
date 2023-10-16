use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod basic_auth_settings;

pub use basic_auth_settings::*;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
    pub(crate) application_id: Uuid,
    pub(crate) basic_auth_settings_id: Uuid,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NewApplication {
    pub(crate) application_id: Uuid,
    pub(crate) basic_auth_settings_id: Uuid,
}