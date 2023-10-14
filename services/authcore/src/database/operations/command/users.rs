use crate::database::{
    models::{NewUser, User},
    Pool,
};

pub struct UserCommands {}

impl UserCommands {
    pub async fn insert(pool: &Pool, user: NewUser) -> Result<(), sqlx::Error> {
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

    pub async fn insert_returning(pool: &Pool, user: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (user_id, application_id, external_id, primary_email_id, full_name, display_name)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
            "#,
            user.user_id,
            user.application_id,
            user.external_id,
            user.primary_email_id,
            user.full_name,
            user.display_name
        )
        .fetch_one(pool)
        .await
    }
}
