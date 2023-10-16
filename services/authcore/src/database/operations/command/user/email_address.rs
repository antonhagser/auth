use crate::database::{models::NewEmailAddress, Pool};

pub struct EmailAddressCommands {}

impl EmailAddressCommands {
    pub async fn create(pool: &Pool, new: NewEmailAddress) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO email_addresses (email_id, user_id, application_id, email, is_primary, is_verified, verified_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            new.email_id,
            new.user_id,
            new.application_id,
            new.email,
            new.is_primary,
            new.is_verified,
            new.verified_at
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
