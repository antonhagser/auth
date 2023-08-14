use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Provide password requirements configuration to organizations (e.g. min length, max length, etc.)
///
/// Please do not recommend organizations to use password requirements that are too strict. It's ridiculous and annoying.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct PasswordRequirements {
    /// Minimum password length
    pub min_length: u8,
    /// Maximum password length
    pub max_length: u8,

    /// Enable strict requirements. If enabled, the password must contain at least one lowercase letter, one uppercase letter, one number, and one symbol.
    pub enable_strict_requirements: bool,
    /// Minimum lowercase letters
    pub min_lowercase: u8,
    /// Minimum uppercase letters
    pub min_uppercase: u8,
    /// Minimum numbers
    pub min_numbers: u8,
    /// Minimum symbols
    pub min_symbols: u8,

    /// Minimum zxcvbn score required for password
    pub min_zxcvbn_score: u8,
}

impl Default for PasswordRequirements {
    fn default() -> Self {
        Self {
            enable_strict_requirements: false,
            /// Recommended by NIST SP 800-63B
            min_length: 8,
            max_length: 128,
            min_lowercase: 0,
            min_uppercase: 0,
            min_numbers: 0,
            min_symbols: 0,
            min_zxcvbn_score: 3,
        }
    }
}

#[derive(Debug, Clone, Error, PartialEq, Serialize)]
pub enum PasswordValidationError {
    #[error("invalid length, got {0}, expected min {1}")]
    MinLength(usize, usize),
    #[error("invalid length, got {0}, expected max {1}")]
    MaxLength(usize, usize),
    #[error("invalid lowercase, got {0}, expected {1}")]
    LowercaseCount(usize, usize),
    #[error("invalid uppercase, got {0}, expected {1}")]
    UppercaseCount(usize, usize),
    #[error("invalid number count, got {0}, expected {1}")]
    NumberCount(usize, usize),
    #[error("invalid symbol count, got {0}, expected {1}")]
    SymbolCount(usize, usize),
    #[error("password not strong enough")]
    NotStrongEnough,
}

/// Validates a password according to the provided requirements.
///
/// This function checks if the given password meets the requirements specified in the `requirements` parameter.
/// It also checks if the password contains any of the strings provided in the `user_inputs` parameter, which
/// might include things like the user's username or email address.
///
/// # Arguments
///
/// * `requirements` - A `PasswordRequirements` object that specifies the rules the password must follow.
/// * `password` - A string slice that contains the password to validate.
/// * `user_inputs` - A vector of strings that the password should not contain. (ex. email)
pub fn validate_password(
    password: &str,
    user_inputs: &[&str],
    check_strength: bool,
    requirements: PasswordRequirements,
) -> Result<(), Vec<PasswordValidationError>> {
    // validate password, return all errors
    let mut valid_password = true;
    let mut errors = Vec::new();

    // Check against min length
    if password.len() < requirements.min_length as usize {
        valid_password = false;
        errors.push(PasswordValidationError::MinLength(
            password.len(),
            requirements.min_length as usize,
        ));
    }

    // Check against max length
    if password.len() > requirements.max_length as usize {
        valid_password = false;
        errors.push(PasswordValidationError::MaxLength(
            password.len(),
            requirements.max_length as usize,
        ));
    }

    // If strict requirements are enabled, check against lowercase, uppercase, numbers, and symbols
    if requirements.enable_strict_requirements {
        let mut lowercase_count = 0;
        let mut uppercase_count = 0;
        let mut number_count = 0;
        let mut symbol_count = 0;

        for c in password.chars() {
            if c.is_lowercase() {
                lowercase_count += 1;
            } else if c.is_uppercase() {
                uppercase_count += 1;
            } else if c.is_numeric() {
                number_count += 1;
            } else {
                symbol_count += 1;
            }
        }

        // Check against lowercase
        if lowercase_count < requirements.min_lowercase as usize {
            valid_password = false;
            errors.push(PasswordValidationError::LowercaseCount(
                lowercase_count,
                requirements.min_lowercase as usize,
            ));
        }

        // Check against uppercase
        if uppercase_count < requirements.min_uppercase as usize {
            valid_password = false;
            errors.push(PasswordValidationError::UppercaseCount(
                uppercase_count,
                requirements.min_uppercase as usize,
            ));
        }

        // Check against numbers
        if number_count < requirements.min_numbers as usize {
            valid_password = false;
            errors.push(PasswordValidationError::NumberCount(
                number_count,
                requirements.min_numbers as usize,
            ));
        }

        // Check against symbols
        if symbol_count < requirements.min_symbols as usize {
            valid_password = false;
            errors.push(PasswordValidationError::SymbolCount(
                symbol_count,
                requirements.min_symbols as usize,
            ));
        }
    }

    // ! check against dropbox zxcvbn
    // ! Currently locked to score 2 due to inaccuracy of zxcvbn in rust compared to the typescript version (Is WASM an option?)
    if check_strength {
        if let Ok(r) = zxcvbn::zxcvbn(password, user_inputs) {
            if r.score() < 2 {
                valid_password = false;
                errors.push(PasswordValidationError::NotStrongEnough);
            }
        }
    }

    // If password is not valid, return all errors
    if !valid_password {
        return Err(errors);
    }

    Ok(())
}
