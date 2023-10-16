use crate::database::{models::NewApplication, Pool};

mod basic_auth_settings;

pub use basic_auth_settings::BasicAuthSettingsCommands;

pub struct ApplicationCommands {}

impl ApplicationCommands {
    pub async fn create(pool: &Pool, application: NewApplication) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO applications (application_id, basic_auth_settings_id)
            VALUES ($1, $2)
            "#,
            application.application_id,
            application.basic_auth_settings_id
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
