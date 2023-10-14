use sqlx::postgres::PgPoolOptions;
use tracing::{info, trace};

use crate::DATABASE_URL;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub mod models;
pub mod operations;

pub async fn init_pool() -> Result<Pool, sqlx::error::Error> {
    trace!("initializing database pool");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL.as_str())
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &Pool) -> Result<(), sqlx::error::Error> {
    info!("running database migrations");
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
