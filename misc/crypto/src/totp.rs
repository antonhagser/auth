//! Time-based One-Time Password (TOTP) implementation.
//! The TOTP algorithm is based on the HOTP (HMAC-based One-Time Password) algorithm but uses time as the moving factor.

use chrono::{DateTime, Utc};
pub use data_encoding::BASE32_NOPAD;
use hmac::{Hmac, Mac};

use sha1::Sha1;

/// An error type for TOTP-related errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error indicating that the provided secret is invalid.
    #[error("Invalid secret")]
    InvalidSecret,
}

impl From<data_encoding::DecodeError> for Error {
    fn from(_: data_encoding::DecodeError) -> Self {
        Error::InvalidSecret
    }
}

impl From<sha1::digest::InvalidLength> for Error {
    fn from(_: sha1::digest::InvalidLength) -> Self {
        Error::InvalidSecret
    }
}

/// Generates a Time-based One-Time Password (TOTP) using the provided secret and interval.
///
/// The TOTP algorithm is based on the HOTP (HMAC-based One-Time Password) algorithm
/// but uses time as the moving factor. This function takes a secret and an interval,
/// then generates a TOTP based on the current time.
///
/// # Arguments
///
/// * `secret` - A byte slice representing the secret key, usually shared between the server and the client.
/// * `interval` - A `u64` representing the time interval (in seconds) during which the generated TOTP is valid.
///
/// # Examples
///
/// ```
/// use data_encoding::BASE32_NOPAD;
/// use crypto::totp::generate_totp;
///
/// let secret = "JBSWY3DPEHPK3PXP";
/// let secret = BASE32_NOPAD.encode(secret.as_bytes());
/// let interval = 30;
///
/// let totp = generate_totp(&secret.as_bytes(), interval).unwrap();
/// println!("Generated TOTP: {}", totp);
/// ```
///
/// # Returns
///
/// A `Result<String, totp::Error>` containing the generated TOTP or an error.
pub fn generate_totp(secret: &[u8], interval: u32) -> Result<String, Error> {
    let secret = BASE32_NOPAD.decode(secret)?; // Decode the secret to bytes

    let utc: DateTime<Utc> = Utc::now();
    let current_time = utc.timestamp() as u64;
    let counter = current_time / interval as u64;

    generate_hotp(&secret, counter)
}

/// Generates an HMAC-based One-Time Password (HOTP) using the provided secret and counter.
///
/// The HOTP algorithm is a widely-used method for generating one-time passwords based on
/// an HMAC (Hash-based Message Authentication Code) function. This function takes a secret
/// and a counter as inputs and generates an HOTP.
///
/// # Arguments
///
/// * `secret` - A byte slice representing the secret key, usually shared between the server and the client.
/// * `counter` - A `u64` representing the counter value, which increases each time a new HOTP is generated.
///
/// # Examples
///
/// ```
/// use crypto::totp::generate_hotp;
///
/// let secret = b"mysecretkey";
/// let counter = 42;
/// let hotp = generate_hotp(secret, counter).unwrap();
/// println!("Generated HOTP: {}", hotp);
/// ```
///
/// # Returns
///
/// A `Result<String, Error>` containing the generated HOTP or an error.
pub fn generate_hotp(secret: &[u8], counter: u64) -> Result<String, Error> {
    let mut mac = Hmac::<Sha1>::new_from_slice(secret)?;
    mac.update(&counter.to_be_bytes());
    let result = mac.finalize().into_bytes();

    let offset = (result[19] & 0xF) as usize;
    let truncated_hash = u32::from_be_bytes([
        result[offset] & 0x7F,
        result[offset + 1],
        result[offset + 2],
        result[offset + 3],
    ]);

    let otp = format!("{:06}", truncated_hash % 1_000_000);
    Ok(otp)
}

/// Verifies a Time-based One-Time Password (TOTP) using the provided input, secret, interval, and tolerance.
///
/// This function checks if the provided TOTP is valid for the given secret and time interval, considering
/// an optional tolerance value for time synchronization. It generates a range of expected TOTPs based on the
/// tolerance and verifies if the input TOTP matches any of them.
///
/// # Arguments
///
/// * `input_totp` - A `String` representing the TOTP to be verified.
/// * `secret` - A byte slice representing the secret key (encoded with base32), usually shared between the server and the client.
/// * `interval` - A `u64` representing the time interval (in seconds) during which the generated TOTP is valid.
/// * `tolerance` - An `Option<u64>` representing the number of intervals to allow as a tolerance for time synchronization.
///                 If not provided, the tolerance is assumed to be zero.
///
/// # Examples
///
/// ```
/// use data_encoding::BASE32_NOPAD;
/// use crypto::totp::verify_totp;
///
/// let secret = "JBSWY3DPEHPK3PXP";
/// let secret = BASE32_NOPAD.encode(secret.as_bytes());
/// let interval = 30;
/// let tolerance = Some(1);
/// let input_totp = "123456";
///
/// let is_valid = verify_totp(input_totp, &secret.as_bytes(), interval, tolerance).unwrap();
/// println!("Is the TOTP valid? {}", is_valid);
/// ```
///
/// # Returns
///
/// A `Result<bool, totp::Error>` result either containing a bool indicating whether the input TOTP is valid or not or a error.
pub fn verify_totp(
    input_totp: &str,
    secret: &[u8],
    interval: u32,
    tolerance: Option<u64>,
) -> Result<bool, Error> {
    let secret = BASE32_NOPAD.decode(secret)?; // Decode the secret to bytes

    let utc: DateTime<Utc> = Utc::now();
    let current_time = utc.timestamp() as u64;
    let counter = current_time / interval as u64;

    let tolerance = tolerance.unwrap_or(0);

    for i in (counter.saturating_sub(tolerance))..=(counter + tolerance) {
        let expected_totp = generate_hotp(&secret, i)?;
        if input_totp == expected_totp {
            return Ok(true);
        }
    }

    Ok(false)
}
