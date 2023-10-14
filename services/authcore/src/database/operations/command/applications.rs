use crate::database::{models::Application, Pool};

pub struct ApplicationCommands {}

impl ApplicationCommands {
    pub async fn insert(pool: &Pool, application: Application) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO applications (application_id)
            VALUES ($1)
            "#,
            application.application_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
