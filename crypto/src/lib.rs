//! crypto is a Rust library that provides cryptographic utilities, such as password hashing and verification
//! using the Argon2 algorithm, as well as Time-based One-Time Password (TOTP) generation and verification.
//!
//! # Usage
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! crypto = "0.1.0"
//! ```
//!
//! Then, add the following to your main Rust file:
//!
//! ```
//! use crypto::password;
//! use crypto::totp;
//! ```
//!
//! # Examples
//!
//! ## Password Hashing and Verification
//!
//! ```
//! use crypto::password::{hash_and_salt_password, verify_password};
//!
//! let password = "my_password";
//! let password_hash = hash_and_salt_password(password).unwrap();
//!
//! assert!(verify_password(password, &password_hash).unwrap());
//! ```
//!
//! ## TOTP Generation and Verification
//!
//! ```
//! use data_encoding::BASE32_NOPAD;
//! use crypto::totp::{generate_totp, verify_totp};
//!
//! let secret = "JBSWY3DPEHPK3PXP";
//! let secret = BASE32_NOPAD.encode(secret.as_bytes());
//! let interval = 30;
//! let tolerance = Some(1);
//!
//! let totp = generate_totp(&secret.as_bytes(), interval).unwrap();
//! let is_valid = verify_totp(&totp, &secret.as_bytes(), interval, tolerance).unwrap();
//!
//! assert!(is_valid);
//! ````

pub mod jsonwebtoken;
pub mod paseto;
pub mod password;
pub mod snowflake;
pub mod totp;
