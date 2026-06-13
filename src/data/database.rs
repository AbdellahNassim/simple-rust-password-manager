use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn create_pool(pool_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(pool_path)?.create_if_missing(true);
    SqlitePool::connect_with(options).await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    MIGRATOR.run(pool).await?;
    Ok(())
}
