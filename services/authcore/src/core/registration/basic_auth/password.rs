use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;
use validator::{Validate, ValidationErrors};

/// Provide password requirements configuration to organizations (e.g. min length, max length, etc.)
///
/// Please do not recommend organizations to use password requirements that are too strict. It's ridiculous and annoying.
#[derive(Debug, Clone, Copy, Validate, Deserialize, PartialEq, Eq)]
pub struct PasswordRequirements {
    /// Enable strict requirements. If enabled, the password must contain at least one lowercase letter, one uppercase letter, one number, and one symbol.
    pub(crate) enable_strict_requirements: bool,
    #[validate(range(min = 8, max = 128))]
    pub(crate) min_length: u8,
    #[validate(range(min = 8, max = 128))]
    pub(crate) max_length: u8,
    pub(crate) min_lowercase: u8,
    pub(crate) min_uppercase: u8,
    pub(crate) min_numbers: u8,
    pub(crate) min_symbols: u8,

    /// Minimum zxcvbn score required for password
    #[validate(range(min = 0, max = 4))]
    pub(crate) min_zxcvbn_score: u8,
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
            min_zxcvbn_score: 2,
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

    #[error("invalid password requirements")]
    #[serde(skip)]
    InvalidPasswordRequirements(#[from] ValidationErrors),
}

pub fn validate_password(
    requirements: PasswordRequirements,
    password: &str,
    user_inputs: Vec<String>,
) -> Result<(), Vec<PasswordValidationError>> {
    // validate requirements
    if let Err(e) = requirements.validate() {
        error!("invalid password requirements: {}", e);
        return Err(vec![PasswordValidationError::InvalidPasswordRequirements(
            e,
        )]);
    }

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

    // check against dropbox zxcvbn
    let user_inputs_str: Vec<&str> = user_inputs.iter().map(|s| s.as_str()).collect();
    if let Ok(r) = zxcvbn::zxcvbn(password, &user_inputs_str) {
        if r.score() < requirements.min_zxcvbn_score {
            valid_password = false;
            errors.push(PasswordValidationError::NotStrongEnough);
        }
    }

    // If password is not valid, return all errors
    if !valid_password {
        return Err(errors);
    }

    Ok(())
}
