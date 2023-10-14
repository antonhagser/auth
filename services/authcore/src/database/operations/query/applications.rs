use uuid::Uuid;

use crate::database::{models::Application, Pool};

pub struct ApplicationQueries {}

impl ApplicationQueries {
    pub async fn find_by_id(pool: &Pool, id: Uuid) -> Result<Application, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"
            SELECT * FROM applications
            WHERE application_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }
}
