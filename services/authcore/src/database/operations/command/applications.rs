use crate::database::{
    models::{Application, NewApplication},
    Pool,
};

pub struct ApplicationCommands {}

impl ApplicationCommands {
    pub async fn insert(pool: &Pool, application: NewApplication) -> Result<(), sqlx::Error> {
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

    pub async fn insert_returning(
        pool: &Pool,
        application: NewApplication,
    ) -> Result<Application, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"
            INSERT INTO applications (application_id, basic_auth_settings_id)
            VALUES ($1, $2) RETURNING *
            "#,
            application.application_id,
            application.basic_auth_settings_id
        )
        .fetch_one(pool)
        .await
    }
}
