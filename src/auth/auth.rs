use std::io::{self, Write};

use crate::data::sqlite_repository::SqliteCredentialRepository;
use crate::errors::AppError;
use crate::crypto::CryptoService;
use crate::services::commands::read_password_input;
use crate::master_password::verify_master_password;

pub async  fn authenticate(repository: &mut SqliteCredentialRepository) -> Result<CryptoService, AppError> {

    let config = repository.get_vault_config().await.map_err(|_| AppError::FailedToGetVaultConfig)?;
    if config.is_none() {
        return Err(AppError::VaultNotSetup);
    }
    let config = config.unwrap();
    print!("Enter master password: ");
    io::stdout().flush().unwrap();
    let master_password = read_password_input()?;
    
    let checked = verify_master_password(&master_password, &config.password_hash).map_err(|_| AppError::InvalidMasterPassword)?;

    if !checked {
        return Err(AppError::InvalidMasterPassword);
    }

    let crypto_service = CryptoService::new(&master_password, &config.salt).map_err(|_| AppError::FailedToCreateCryptoService)?;
    Ok(crypto_service)
}