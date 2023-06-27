//! A module for handling password hashing and verification using the Argon2 password hashing
//! algorithm with salt generation.

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;
use thiserror::Error;

/// An error type for hashing and verification of passwords.
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid password hash: {0}")]
    InvalidPasswordHash(String, argon2::password_hash::Error),

    #[error("Hash Error")]
    HashError(argon2::password_hash::Error),

    #[error("not matching")]
    NotMatching,
}

/// Hashes and salts a password using Argon2, and returns the resulting hash as a PHC-formatted string.
///
/// # Arguments
///
/// * `password` - The password to be hashed and salted.
///
/// # Example
///
/// ```
/// use crypto::password::hash_and_salt_password;
///
/// let password = "password";
/// let password_hash = hash_and_salt_password(password).unwrap();
/// ```
pub fn hash_and_salt_password(password: &str) -> Result<String, CryptoError> {
    // Generate salt
    let salt = SaltString::generate(&mut OsRng);

    // Default argon configuration
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(CryptoError::HashError)?
        .to_string();

    Ok(password_hash)
}

/// Verifies a password against a given password hash. Returns () if the password matches the hash.
///
/// # Arguments
///
/// * `password` - The password to be verified.
/// * `password_hash` - The PHC-formatted password hash string to verify against.
pub fn verify_password(password: &str, password_hash: &str) -> Result<(), CryptoError> {
    // Default argon configuration
    let argon2 = Argon2::default();

    let hash = PasswordHash::new(password_hash)
        .map_err(|e| CryptoError::InvalidPasswordHash(password_hash.to_string(), e))?;

    // Verify password
    let is_valid = argon2.verify_password(password.as_bytes(), &hash).is_ok();
    if !is_valid {
        return Err(CryptoError::NotMatching);
    }

    Ok(())
}
