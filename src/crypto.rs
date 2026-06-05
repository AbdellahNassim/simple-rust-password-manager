use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};

use base64::{Engine as _, engine::general_purpose::STANDARD};

use argon2::Argon2;

pub struct CryptoService {
    cipher: Aes256Gcm,
}

impl CryptoService {
    pub fn new(master_password: &str, salt: &str) -> Result<Self, String> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(master_password.as_bytes(), salt.as_bytes(), &mut key)
            .map_err(|e| e.to_string())?;
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;
        Ok(Self { cipher })
    }
    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
    
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| e.to_string())?;
        let mut combined = nonce.to_vec();
        combined.extend(ciphertext);
    
        let encoded = STANDARD.encode(combined);
        Ok(encoded)
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, String> {
        let bytes = STANDARD.decode(ciphertext).map_err(|e| e.to_string())?;
        let (nonce_bytes, ciphertext) = bytes.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8(plaintext).map_err(|e| e.to_string())?)   
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = "Hello, world!";
        let master_password = "password";
        let salt = "vault-salt";
        let crypto_service = CryptoService::new(master_password, salt).unwrap();
        let encrypted = crypto_service.encrypt(plaintext).unwrap();
        let decrypted = crypto_service.decrypt(&encrypted).unwrap();
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_master_passwords_fail() {
        let salt = "vault-salt";
        let crypto1 =
            CryptoService::new(
                "password-one",
                salt,
            )
            .unwrap();

        let crypto2 =
            CryptoService::new(
                "password-two",
                salt,
            )
            .unwrap();

        let encrypted =
            crypto1
                .encrypt("secret")
                .unwrap();

        assert!(
            crypto2
                .decrypt(&encrypted)
                .is_err()
        );
}
}
