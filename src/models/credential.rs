use std::fmt::Display;
use crate::errors::AppError;
#[derive(Debug,Clone,PartialEq)]
pub struct Credential {
    pub id: u32,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn new(id: u32, service: String, username: String, password: String) -> Result<Self,AppError> {
        if service.trim().is_empty() {
            return Err(AppError::EmptyService);
        }
        if username.trim().is_empty() {
            return Err(AppError::EmptyUsername);
        }
        if password.trim().is_empty() {
            return Err(AppError::EmptyPassword);
        }
        Ok(Self { id, service, username, password })
    }
}

impl Display for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Credential {{ id: {}, service: {}, username: {}, password: {} }}", self.id, self.service, self.username, self.password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_credential() {
        let credential = Credential::new(1, "test".to_string(), "test".to_string(), "test".to_string()).unwrap();
        assert_eq!(credential.id, 1);
        assert_eq!(credential.service, "test");
        assert_eq!(credential.username, "test".to_string());
        assert_eq!(credential.password, "test".to_string());
    }

    #[test]
    fn test_new_credential_empty_service() {
        let result = Credential::new(1, "".to_string(), "test".to_string(), "test".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::EmptyService));
    }

    #[test]
    fn test_new_credential_empty_username() {
        let result = Credential::new(1, "test".to_string(), "".to_string(), "test".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::EmptyUsername));
    }

    #[test]
    fn test_new_credential_empty_password() {
        let result = Credential::new(1, "test".to_string(), "test".to_string(), "".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::EmptyPassword));
    }
}