use base64::{Engine, engine::general_purpose};
use argon2::{
    password_hash::{
        PasswordHasher,SaltString, PasswordHash, PasswordVerifier
    },
    Argon2
};

pub fn generate_salt() -> String {
    let salt : [u8; 16] = rand::random();
    general_purpose::STANDARD.encode(salt)
}

pub fn hash_master_password(password: &str, salt: &str) -> Result<String, String> {
    let salt = SaltString::encode_b64(salt.as_bytes()).map_err(|e| e.to_string())?;
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).map_err(|e| e.to_string())?;
    Ok(password_hash.to_string())
}

pub fn verify_master_password(password: &str, hash: &str) -> Result<bool, String> {
   let parsed = PasswordHash::new(hash);
   let argon2 = Argon2::default();

   match parsed {
    Ok(hash) => {
        Ok(argon2.verify_password(password.as_bytes(), &hash).is_ok())
    },
    Err(e) => {
        Err(e.to_string())
    }
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_salt() {
        let salt = generate_salt();
        assert!(!salt.is_empty());
    }

    #[test]
    fn test_hash_and_verify_master_password() {
        let password = "password";
        let salt = generate_salt();
        let hash = hash_master_password(password, &salt).unwrap();
        let result = verify_master_password(password, &hash).unwrap();
        assert!(result);
    }

    #[test]
    #[should_panic]
    fn test_verify_invalid_hash() {
        let password = "password";
        let hash = "invalid_hash";
        let result = verify_master_password(password, hash).expect("Failed to verify master password");

    }
    #[test]
    fn test_hash_and_verify() {
        let salt =
            generate_salt();

        let hash =
            hash_master_password(
                "super-secret",
                &salt,
            )
            .unwrap();

        assert!(
            verify_master_password(
                "super-secret",
                &hash
            ).unwrap()
        );

        assert!(
            !verify_master_password(
                "wrong-password",
                &hash
            ).unwrap()
        );
    }
}