use crate::database::{models::NewUser, Pool};

mod basic_auth;
mod email_address;

pub use basic_auth::BasicAuthCommands;
pub use email_address::EmailAddressCommands;

pub struct UserCommands {}

impl UserCommands {
    pub async fn create(pool: &Pool, user: NewUser) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (user_id, application_id, external_id, primary_email_id, full_name, display_name)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user.user_id,
            user.application_id,
            user.external_id,
            user.primary_email_id,
            user.full_name,
            user.display_name
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
