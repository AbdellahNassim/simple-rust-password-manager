use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite:vault.db")?.create_if_missing(true);
    SqlitePool::connect_with(options).await
}

pub async fn initialize_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS credentials (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            service TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        "#
    ).execute(pool).await?;


    Ok(())
}