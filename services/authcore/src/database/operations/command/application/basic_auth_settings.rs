use crate::database::{models::NewBasicAuthSettings, Pool};

pub struct BasicAuthSettingsCommands {}

impl BasicAuthSettingsCommands {
    pub async fn create(
        pool: &Pool,
        new_basic_auth_settings: NewBasicAuthSettings,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO basic_auth_settings (application_id, min_password_length, max_password_length, require_lowercase, require_uppercase, require_numeric, require_special, password_history_count, password_expiry_days, max_failed_attempts, lockout_duration, require_mfa)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            new_basic_auth_settings.application_id,
            new_basic_auth_settings.min_password_length,
            new_basic_auth_settings.max_password_length,
            new_basic_auth_settings.require_lowercase,
            new_basic_auth_settings.require_uppercase,
            new_basic_auth_settings.require_numeric,
            new_basic_auth_settings.require_special,
            new_basic_auth_settings.password_history_count,
            new_basic_auth_settings.password_expiry_days,
            new_basic_auth_settings.max_failed_attempts,
            new_basic_auth_settings.lockout_duration,
            new_basic_auth_settings.require_mfa
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}
