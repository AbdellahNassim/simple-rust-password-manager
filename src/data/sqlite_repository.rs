use crate::data::repository::CredentialRepository;
use crate::errors::AppError;
use crate::models::credential::Credential;
use sqlx::SqlitePool;

pub struct SqliteCredentialRepository {
    pool: SqlitePool,
}

impl SqliteCredentialRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add(&self, credential: &Credential) -> Result<(), sqlx::Error> {

        sqlx::query(
            r#"
            INSERT INTO credentials (service, username, password) VALUES (?, ?, ?);
            "#
        )
        .bind(&credential.service)
        .bind(&credential.username)
        .bind(&credential.password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Credential>, sqlx::Error> {
        sqlx::query_as::<_, Credential>(
            r#"
            SELECT * FROM credentials;
            "#
        ).fetch_all(&self.pool)
        .await
    }
}