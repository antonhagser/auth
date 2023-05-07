//! This module implements a JSON Web Token (JWT) handler for authentication purposes.
//!
//! It provides methods for generating and verifying JWTs, including expiration checks.
//! The tokens are signed using the RS256 algorithm.
//!
//! # Example
//!
//! ```
//! use crypto::jwt::{JWT, Claims};
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
//!
//! // Generate a token
//! let token = JWT::generate_token(id, &priv_key_pem).unwrap();
//!
//! // Verify the token
//! let claims = JWT::verify_token(&token, &pub_key_pem).unwrap();
//! assert_eq!(claims.sub(), id);
//! ```

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

/// Represents the payload of a JWT, containing the subject (sub), issuer (iss), and expiration (exp).
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iss: String,
    exp: usize,
}

impl Claims {
    /// Returns the subject (sub) of the JWT.
    pub fn sub(&self) -> &str {
        self.sub.as_ref()
    }

    /// Returns the issuer (iss) of the JWT.
    pub fn iss(&self) -> &str {
        self.iss.as_ref()
    }

    /// Returns the expiration (exp) of the JWT.
    pub fn exp(&self) -> usize {
        self.exp
    }
}

/// A struct representing the JSON Web Token handler.
pub struct JWT {}

impl JWT {
    /// Generates a JWT with a given `id`, using the provided RSA private key in PEM format.
    ///
    /// The JWT will be signed using the RS256 algorithm.
    /// The expiration of the JWT is set to 90 days from the current time.
    ///
    /// # Arguments
    ///
    /// * `id` - The subject (sub) for the JWT.
    /// * `priv_key_pem` - A byte slice containing the RSA private key in PEM format.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated JWT as a `String`, or a `jsonwebtoken::errors::Error`
    /// if an error occurred.
    pub fn generate_token<S>(
        id: S,
        priv_key_pem: &[u8],
    ) -> Result<String, jsonwebtoken::errors::Error>
    where
        S: Into<String>,
    {
        let mut header = jsonwebtoken::Header::new(Algorithm::RS256);
        header.kid = Some("authenticator".to_string());

        let claims = Claims {
            sub: id.into(),
            iss: "authenticator".to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::days(90)).timestamp() as usize,
        };

        jsonwebtoken::encode(&header, &claims, &EncodingKey::from_rsa_pem(priv_key_pem)?)
    }

    /// Verifies a JWT using the provided RSA public key in PEM format.
    ///
    /// The JWT is expected to be signed with the RS256 algorithm.
    /// This method also checks if the JWT has expired.
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT as a `&str`.
    /// * `pub_key_pem` - A byte slice containing the RSA public key in PEM format.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded JWT claims as a `Claims` struct, or a
    /// `jsonwebtoken::errors::Error` if an error occurred.
    pub fn verify_token(
        token: &str,
        pub_key_pem: &[u8],
    ) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_rsa_pem(pub_key_pem)?,
            &jsonwebtoken::Validation::new(Algorithm::RS256),
        )
        .map(|data| data.claims);

        match token {
            Ok(token) => {
                // check if the token has expired
                if token.exp < (chrono::Utc::now()).timestamp() as usize {
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
