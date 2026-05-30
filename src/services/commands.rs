pub fn add_credential(service: String, username: String) {
    println!("Adding a new password for {} with username {}", service, username);
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