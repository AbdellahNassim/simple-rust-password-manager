use rpassword::read_password;
use std::io::{self, Write};
use crate::errors::AppError;
use crate::models::credential::Credential;
use crate::data::repository::CredentialRepository;
use crate::crypto::{CryptoService};

fn read_password_input() -> Result<String, AppError> {
    read_password().map_err(|_| AppError::FailedToReadPassword)
}

pub async fn add_credential<R>(repository: &mut R, service: String, username: String) -> Result<(), AppError> 
where R: CredentialRepository + Send + Sync + 'static
{
    
print!("Enter password for {}: ", service);
io::stdout().flush().unwrap();
let password = read_password_input()?;
let crypto_service = CryptoService::new("password").map_err(|_| AppError::EncryptionError)?;
let encrypted_password = crypto_service.encrypt(&password).map_err(|_| AppError::EncryptionError)?;
let credential = Credential::new(1, service, username, encrypted_password)?;

repository.add_credential(credential).await.map_err(|_| AppError::FailedToAddCredential)?;
println!("Credential saved successfully");
println!("--------------------------------");
Ok(())
}

pub async fn get_credential<R>(repository: &R, service: String) -> Result<(), AppError>
where R: CredentialRepository + Send + Sync + 'static
{
    println!("Getting a password for {}", service);
    println!("--------------------------------");
    let credential = repository.get_credential_by_service(service).await.map_err(|_| AppError::FailedToGetCredential)?;
    if credential.is_none() {
        println!("Credential not found.");
        println!("--------------------------------");
        return Err(AppError::CredentialNotFound);
    }
    let borrowed_credential = credential.as_ref().unwrap();
    println!("{}: {}", borrowed_credential.service, borrowed_credential.username);
    let crypto_service = CryptoService::new("password").map_err(|_| AppError::EncryptionError)?;
    let decrypted_password = crypto_service.decrypt(&borrowed_credential.password).map_err(|_| AppError::DecryptionError)?;
    println!("Password: {}", decrypted_password);
    println!("--------------------------------");
    Ok(())
}

pub async fn delete_credential<R>(repository: &mut R, service: String) -> Result<(), AppError>
where R: CredentialRepository + Send + Sync + 'static
{
    println!("Removing a password for {}", service);
    println!("--------------------------------");
    let result = repository.delete_credential(service).await.map_err(|_| AppError::FailedToDeleteCredential)?;
    if !result {
        println!("Credential not found.");
        println!("--------------------------------");
        return Err(AppError::CredentialNotFound);
    }
    println!("Credential deleted successfully");
    println!("--------------------------------");
    Ok(())
}

pub async fn list_credentials<R>(repository: &R) -> Result<(), AppError>
where R: CredentialRepository + Send + Sync + 'static
{
    let credentials = repository.list_credentials().await.map_err(|_| AppError::FailedToListCredentials)?;
    println!("Listing all passwords");
    println!("--------------------------------");
    if credentials.is_empty() {
        println!("No credentials found.");
        println!("--------------------------------");
        return Ok(());
    }
    for credential in credentials {
        println!("{}: {}", credential.service, credential.username);
    }
    println!("--------------------------------");
    Ok(())
}