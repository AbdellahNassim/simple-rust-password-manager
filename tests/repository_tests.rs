use password_manager::data::database::{create_pool, run_migrations};
use password_manager::data::repository::CredentialRepository;
use password_manager::data::sqlite_repository::SqliteCredentialRepository;
use password_manager::models::credential::Credential;

#[tokio::test]
async fn test_save_and_load_credential() {
    let pool = create_pool("sqlite:test.db")
        .await
        .expect("Failed to create database pool");
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");
    let mut repository = SqliteCredentialRepository::new(pool.clone());
    let credential = Credential::new(
        1,
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
    )
    .unwrap();
    repository.add_credential(credential.clone()).await.unwrap();
    let loaded_credential = repository
        .get_credential_by_service("test".to_string())
        .await
        .unwrap();
    assert_eq!(loaded_credential, Some(credential));
}
