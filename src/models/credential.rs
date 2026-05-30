use std::fmt::Display;

#[derive(Debug,Clone,PartialEq)]
pub struct Credential {
    pub id: u32,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn new(id: u32, service: String, username: String, password: String) -> Self {
        
        assert!(
            !service.trim().is_empty(),
            "Service cannot be empty"
        );
        assert!(
            !username.trim().is_empty(),
            "Username cannot be empty"
        );
        assert!(
            !password.trim().is_empty(),
            "Password cannot be empty"
        );
        
        Self { id, service, username, password }
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
        let credential = Credential::new(1, "test".to_string(), "test".to_string(), "test".to_string());
        assert_eq!(credential.id, 1);
        assert_eq!(credential.service, "test");
        assert_eq!(credential.username, "test".to_string());
        assert_eq!(credential.password, "test".to_string());
    }

    #[test]
    #[should_panic(expected = "Service cannot be empty")]
    fn test_new_credential_empty_service() {
        Credential::new(1, "".to_string(), "test".to_string(), "test".to_string());
    }

    #[test]
    #[should_panic(expected = "Username cannot be empty")]
    fn test_new_credential_empty_username() {
        Credential::new(1, "test".to_string(), "".to_string(), "test".to_string());
    }

    #[test]
    #[should_panic(expected = "Password cannot be empty")]
    fn test_new_credential_empty_password() {
        Credential::new(1, "test".to_string(), "test".to_string(), "".to_string());
    }
}