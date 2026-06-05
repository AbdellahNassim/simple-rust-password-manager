#[derive(Debug,Clone,sqlx::FromRow)]
pub struct VaultConfig {
    pub password_hash: String,
    pub salt: String,
}

impl VaultConfig {
    pub fn new(password_hash: String, salt: String) -> Self {
        Self { password_hash, salt }
    }
}