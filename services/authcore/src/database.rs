use sqlx::postgres::PgPoolOptions;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn init_pool() -> Result<Pool, sqlx::error::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:password@localhost/test")
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &Pool) -> Result<(), sqlx::error::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
