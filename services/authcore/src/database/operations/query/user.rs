use uuid::Uuid;

use crate::database::{models::User, Pool};

pub struct UserQueries {}

impl UserQueries {
    pub async fn find_by_id(
        pool: &Pool,
        user_id: Uuid,
        application_id: Uuid,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM Users
            WHERE user_id = $1 AND application_id = $2
            "#,
            user_id,
            application_id
        )
        .fetch_one(pool)
        .await
    }
}
