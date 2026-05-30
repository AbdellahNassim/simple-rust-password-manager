use rpassword::read_password;
use std::io::{self, Write};

use crate::models::credential::Credential;

pub fn add_credential(service: String, username: String) {
    
print!("Enter password for {}: ", service);
io::stdout().flush().unwrap();
let password = read_password().expect("Failed to read password");

let credential = Credential::new(1, service, username, password);

println!();

println!("Credential added: {:#?}", credential);

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