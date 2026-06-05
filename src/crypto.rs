use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

const KEY: &[u8; 32] =
    b"12345678901234567890123456789012";


pub fn encrypt(plaintext: &str) -> Result<String, String> {
    let cipher = Aes256Gcm::new_from_slice(KEY).map_err(|e| e.to_string())?;
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).map_err(|e| e.to_string())?;
    let mut combined = nonce.to_vec();
    combined.extend(ciphertext);

    let encoded = base64::encode(combined);
    Ok(encoded)

}


pub fn decrypt(ciphertext: &str) -> Result<String, String> {
    let cipher = Aes256Gcm::new_from_slice(KEY).map_err(|e| e.to_string())?;

    let bytes = base64::decode(ciphertext).map_err(|e| e.to_string())?;

    let (nonce_bytes,ciphertext) = bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
    Ok(String::from_utf8(plaintext).map_err(|e| e.to_string())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = "Hello, world!";
        let encrypted = encrypt(plaintext).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(plaintext, decrypted);
    }
}
