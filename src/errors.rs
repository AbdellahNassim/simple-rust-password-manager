use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Service cannot be empty")]
    EmptyService,
    #[error("Username cannot be empty")]
    EmptyUsername,
    #[error("Password cannot be empty")]
    EmptyPassword,
    #[error("Credential not found")]
    CredentialNotFound,
    #[error("Failed to read password")]
    FailedToReadPassword,
    #[error("Failed to add credential")]
    FailedToAddCredential,
    #[error("Failed to list credentials")]
    FailedToListCredentials,
    #[error("Failed to get credential")]
    FailedToGetCredential,
    #[error("Failed to delete credential")]
    FailedToDeleteCredential,
    #[error("Failed to encrypt password")]
    EncryptionError,
    #[error("Failed to decrypt password")]
    DecryptionError,
    #[error("Failed to hash master password")]
    FailedToHashMasterPassword,
    #[error("Failed to save vault config")]
    FailedToSaveVaultConfig,
    #[error("Failed to get vault config")]
    FailedToGetVaultConfig,
    #[error("Vault not setup")]
    VaultNotSetup,
    #[error("Invalid master password")]
    InvalidMasterPassword,
    #[error("Failed to create crypto service")]
    FailedToCreateCryptoService,
    #[error("Vault already setup")]
    VaultAlreadySetup,
}