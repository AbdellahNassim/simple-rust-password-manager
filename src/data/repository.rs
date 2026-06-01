use crate::models::credential::Credential;
use std::error::Error;

pub trait CredentialRepository {
    fn add_credential(&mut self, credential: Credential) -> Result<(), Box<dyn Error>>;
    fn get_credential_by_service(&self, service: String) -> Result<Option<&Credential>, Box<dyn Error>>;
    fn delete_credential(&mut self, service: String) -> Result<bool, Box<dyn Error>>;
    fn list_credentials(&self) -> Result<Vec<&Credential>, Box<dyn Error>>;
}   


pub struct InMemoryCredentialRepository {
    credentials: Vec<Credential>,
}

impl  InMemoryCredentialRepository {
    pub fn new() -> Self {
        Self { credentials: Vec::new() }
    }
}

impl CredentialRepository for InMemoryCredentialRepository {
    fn add_credential(&mut self, credential: Credential) -> Result<(), Box<dyn Error>> {
        self.credentials.push(credential);
        Ok(())
    }
    fn get_credential_by_service(&self, service: String) -> Result<Option<&Credential>, Box<dyn Error>> {
        let credential = self.credentials.iter().find(|c| c.service == service);
        Ok(credential)
    }
    fn delete_credential(&mut self, service: String) -> Result<bool, Box<dyn Error>> {
        let initial_length = self.credentials.len();
        self.credentials.retain(|c| c.service != service);
        Ok(initial_length != self.credentials.len())
    }
    fn list_credentials(&self) -> Result<Vec<&Credential>, Box<dyn Error>> {
        Ok(self.credentials.iter().collect())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_credential_in_memory() {
        let mut repository = InMemoryCredentialRepository::new();
        let credential = Credential::new(1, "test".to_string(), "test".to_string(), "test".to_string()).unwrap();
        repository.add_credential(credential).unwrap();
        assert_eq!(repository.credentials.len(), 1);
    }

    #[test]
    fn test_get_credential_by_service_in_memory() {
        let mut repository = InMemoryCredentialRepository::new();
        let credential = Credential::new(1, "test".to_string(), "test".to_string(), "test".to_string()).unwrap();
        repository.add_credential(credential).unwrap();
        let result = repository.get_credential_by_service("test".to_string()).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_delete_credential_in_memory() {
        let mut repository = InMemoryCredentialRepository::new();
        let credential = Credential::new(1, "test".to_string(), "test".to_string(), "test".to_string()).unwrap();
        repository.add_credential(credential).unwrap();
        let result = repository.delete_credential("test".to_string()).unwrap();
        assert!(result);
        assert_eq!(repository.credentials.len(), 0);
    }
}