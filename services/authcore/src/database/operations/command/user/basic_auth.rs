use crate::database::{models::NewBasicAuth, Pool};

pub struct BasicAuthCommands {}

impl BasicAuthCommands {
    pub async fn create(pool: &Pool, new_basic_auth: NewBasicAuth) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO basic_auths (user_id, application_id, password_hash)
            VALUES ($1, $2, $3)
            "#,
            new_basic_auth.user_id,
            new_basic_auth.application_id,
            new_basic_auth.password_hash,
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
