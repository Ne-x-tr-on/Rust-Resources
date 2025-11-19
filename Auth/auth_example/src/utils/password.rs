use argon2::{Argon2, password_hash::{PasswordHasher, PasswordVerifier, SaltString, PasswordHash} };
use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).ok();
    if let Some(parsed) = parsed_hash {
        let argon2 = Argon2::default();
        argon2.verify_password(password.as_bytes(), &parsed).is_ok()
    } else { false }
}
