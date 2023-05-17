use chrono::{DateTime, Utc};
use crypto::snowflake::Snowflake;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExternalUser {
    id: Snowflake,

    user_id: Snowflake,

    provider: String,
    provider_user_id: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
