use crate::data::repository::CredentialRepository;
use crate::models::credential::Credential;
use crate::models::vault_config::VaultConfig;
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::error::Error;

pub struct SqliteCredentialRepository {
    pool: SqlitePool,
}

impl SqliteCredentialRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn save_vault_config(&mut self, vault_config: &VaultConfig) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO vault (id, password_hash, salt) VALUES (1, ?, ?);
            "#,
        )
        .bind(&vault_config.password_hash)
        .bind(&vault_config.salt)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_vault_config(&self) -> Result<Option<VaultConfig>, sqlx::Error> {
        let vault_config = sqlx::query_as::<_, VaultConfig>(
            r#"
            SELECT * FROM vault WHERE id = 1;
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(vault_config)
    }
}

#[async_trait]
impl CredentialRepository for SqliteCredentialRepository {
    async fn add_credential(&mut self, credential: Credential) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            INSERT INTO credentials (service, username, password) VALUES (?, ?, ?);
            "#,
        )
        .bind(&credential.service)
        .bind(&credential.username)
        .bind(&credential.password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_credential_by_service(
        &self,
        service: String,
    ) -> Result<Option<Credential>, Box<dyn Error>> {
        let credential = sqlx::query_as::<_, Credential>(
            r#"
            SELECT * FROM credentials WHERE service = ?;
            "#,
        )
        .bind(service)
        .fetch_optional(&self.pool)
        .await?;

        Ok(credential)
    }

    async fn delete_credential(&mut self, service: String) -> Result<bool, Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            DELETE FROM credentials WHERE service = ?;
            "#,
        )
        .bind(service)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn list_credentials(&self) -> Result<Vec<Credential>, Box<dyn Error>> {
        let credentials = sqlx::query_as::<_, Credential>(
            r#"
            SELECT * FROM credentials;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(credentials)
    }
}
