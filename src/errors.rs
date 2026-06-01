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
    FailedToReadPassword
}