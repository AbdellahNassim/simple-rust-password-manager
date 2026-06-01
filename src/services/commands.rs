use rpassword::read_password;
use std::io::{self, Write};
use crate::errors::AppError;
use crate::models::credential::Credential;


fn read_password_input() -> Result<String, AppError> {
    read_password().map_err(|_| AppError::FailedToReadPassword)
}

pub fn add_credential(service: String, username: String) -> Result<(), AppError> {
    
print!("Enter password for {}: ", service);
io::stdout().flush().unwrap();
let password = read_password_input()?;

let credential = Credential::new(1, service, username, password)?;

println!();

println!("Credential added: {:#?}", credential);

Ok(())
}

pub fn get_credential(service: String) {
    println!("Getting a password for {}", service);
}

pub fn delete_credential(service: String) {
    println!("Removing a password for {}", service);
}

pub fn list_credentials() {
    println!("Listing all passwords");
}