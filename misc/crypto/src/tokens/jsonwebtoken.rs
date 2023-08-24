//! This module implements a JSON Web Token (JWT) handler for authentication purposes.
//!
//! It provides methods for generating and verifying JWTs, including expiration checks.
//! The tokens are signed using the RS256 algorithm.
//!
//! # Example
//!
//! ```
//! use crypto::tokens::jsonwebtoken::{JWT, DefaultClaims, Claims};
//! use rsa::{pkcs8::LineEnding, RsaPrivateKey, RsaPublicKey};
//!
//! let mut rng = rand::thread_rng();
//! let bits = 2048;
//! let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//! let pub_key = RsaPublicKey::from(&priv_key);
//!
//! let priv_key_pem = rsa::pkcs1::EncodeRsaPrivateKey::to_pkcs1_pem(&priv_key, LineEnding::LF)
//!     .expect("failed to serialize private key to PEM")
//!     .as_bytes()
//!     .to_vec();
//!
//! let pub_key_pem = rsa::pkcs1::EncodeRsaPublicKey::to_pkcs1_pem(&pub_key, LineEnding::LF)
//!     .expect("failed to serialize public key to PEM")
//!     .as_bytes()
//!     .to_vec();
//!
//! let id = "user123";
//! let exp_unix = chrono::Utc::now() + chrono::Duration::days(1);
//!
//! let claims: DefaultClaims = DefaultClaims::new(id.to_string(), "authcore".to_string(), exp_unix);
//!
//! // Generate a token
//! let token = JWT::generate_token(claims, &priv_key_pem).unwrap();
//!
//! // Verify the token
//! let claims: DefaultClaims = JWT::verify_token(&token, &pub_key_pem).unwrap();
//! assert_eq!(claims.sub(), id);
//! ```

use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

pub use jsonwebtoken::errors::{Error, ErrorKind};
pub use rsa;

/// Represents the default payload of a JWT, containing the subject (sub), issuer (iss), and expiration (exp).
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultClaims {
    sub: String,
    iss: String,
    exp: usize,
}

impl DefaultClaims {
    /// Creates a new instance of the `DefaultClaims` struct.
    pub fn new(subject: String, issuer: String, expiration: DateTime<Utc>) -> Self {
        Self {
            sub: subject,
            iss: issuer,
            exp: expiration.timestamp() as usize,
        }
    }
}

impl Claims for DefaultClaims {
    /// Returns the subject (sub) of the JWT.
    fn sub(&self) -> &str {
        self.sub.as_ref()
    }

    /// Returns the issuer (iss) of the JWT.
    fn iss(&self) -> &str {
        self.iss.as_ref()
    }

    /// Returns the expiration (exp) of the JWT.
    fn exp(&self) -> usize {
        self.exp
    }
}

/// A struct representing the JSON Web Token handler.
pub struct JWT {}

impl JWT {
    /// Generates a JWT with the provided claims, using the provided RSA private key in PEM format.
    ///
    /// The JWT will be signed using the RS256 algorithm.
    /// The expiration of the JWT is determined by the `exp` field in the provided claims.
    ///
    /// The claims for the JWT will be represented as an instance of the generic
    /// `C` type, which must implement both the `Claims` trait and the `Serialize`
    /// trait.
    ///
    /// # Arguments
    ///
    /// * `claims` - An instance of the `C` type representing the claims for the JWT.
    /// * `priv_key_pem` - A byte slice containing the RSA private key in PEM format.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type used to represent the claims for the JWT. It must implement
    ///         the `Claims` trait and the `Serialize` trait.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated JWT as a `String`, or a `jsonwebtoken::errors::Error`
    /// if an error occurred.
    pub fn generate_token<C>(
        claims: C,
        priv_key_pem: &[u8],
    ) -> Result<String, jsonwebtoken::errors::Error>
    where
        C: Claims + Serialize,
    {
        let mut header = jsonwebtoken::Header::new(Algorithm::RS256);
        header.kid = Some("authenticator".to_string());

        jsonwebtoken::encode(&header, &claims, &EncodingKey::from_rsa_pem(priv_key_pem)?)
    }

    /// Verifies a JWT using the provided RSA public key in PEM format.
    ///
    /// The JWT is expected to be signed with the RS256 algorithm.
    /// This method also checks if the JWT has expired.
    ///
    /// The claims of the decoded JWT will be returned as an instance of the generic
    /// `C` type, which must implement both the `Claims` trait and the `Deserialize`
    /// trait for all lifetimes.
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT as a `&str`.
    /// * `pub_key_pem` - A byte slice containing the RSA public key in PEM format.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type used to represent the decoded JWT claims. It must implement
    ///         the `Claims` trait and the `Deserialize` trait for all lifetimes.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded JWT claims as an instance of the `C` type,
    /// or a `jsonwebtoken::errors::Error` if an error occurred.
    pub fn verify_token<C>(
        token: &str,
        pub_key_pem: &[u8],
    ) -> Result<C, jsonwebtoken::errors::Error>
    where
        C: Claims + for<'de> Deserialize<'de>,
    {
        let token = jsonwebtoken::decode::<C>(
            token,
            &DecodingKey::from_rsa_pem(pub_key_pem)?,
            &jsonwebtoken::Validation::new(Algorithm::RS256),
        )
        .map(|data| data.claims);

        match token {
            Ok(token) => {
                // check if the token has expired
                if token.exp() < (chrono::Utc::now()).timestamp() as usize {
                    Err(jsonwebtoken::errors::Error::from(
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature,
                    ))
                } else {
                    Ok(token)
                }
            }
            Err(e) => Err(e),
        }
    }
}

pub trait Claims {
    fn sub(&self) -> &str;
    fn iss(&self) -> &str;
    fn exp(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::{Claims, DefaultClaims, JWT};
    use rsa::{pkcs8::LineEnding, RsaPrivateKey, RsaPublicKey};

    #[test]
    fn test_generate_token() {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

        let priv_key_pem = rsa::pkcs1::EncodeRsaPrivateKey::to_pkcs1_pem(&priv_key, LineEnding::LF)
            .expect("failed to serialize private key to PEM")
            .as_bytes()
            .to_vec();

        let id = "user123";
        let exp_unix = chrono::Utc::now() + chrono::Duration::days(1);

        let claims: DefaultClaims =
            DefaultClaims::new(id.to_string(), "authcore".to_string(), exp_unix);

        // Generate a token
        let token = JWT::generate_token(claims, &priv_key_pem).unwrap();

        assert!(!token.is_empty());
    }

    #[test]
    fn test_verify_token() {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let priv_key_pem = rsa::pkcs1::EncodeRsaPrivateKey::to_pkcs1_pem(&priv_key, LineEnding::LF)
            .expect("failed to serialize private key to PEM")
            .as_bytes()
            .to_vec();

        let pub_key_pem = rsa::pkcs1::EncodeRsaPublicKey::to_pkcs1_pem(&pub_key, LineEnding::LF)
            .expect("failed to serialize public key to PEM")
            .as_bytes()
            .to_vec();

        let id = "user123";
        let exp_unix = chrono::Utc::now() + chrono::Duration::days(1);

        let claims: DefaultClaims =
            DefaultClaims::new(id.to_string(), "authcore".to_string(), exp_unix);

        // Generate a token
        let token = JWT::generate_token(claims, &priv_key_pem).unwrap();

        // Verify the token
        let claims: DefaultClaims = JWT::verify_token(&token, &pub_key_pem).unwrap();
        assert_eq!(claims.sub(), id);
    }
}
