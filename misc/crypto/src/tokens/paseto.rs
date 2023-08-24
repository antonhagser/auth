//! This module provides utilities for working with Paseto tokens.
//!
//! It defines custom error types, claim structures, and utility functions for
//! generating, encrypting, and validating tokens.

use chrono::{DateTime, Utc};
use rusty_paseto::prelude::{
    AudienceClaim, CustomClaim, ExpirationClaim, IssuerClaim, Key, Local, NotBeforeClaim,
    PasetoBuilder, PasetoParser, PasetoSymmetricKey, SubjectClaim, TokenIdentifierClaim, V4,
};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, Value};

/// Represents possible errors that can occur while working with Paseto tokens.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error that occurs while building a Paseto token.
    #[error("Paseto builder error: {0}")]
    PasetoBuilderError(#[from] rusty_paseto::generic::GenericBuilderError),

    /// Error that occurs while parsing a Paseto token.
    #[error("Paseto claim error: {0}")]
    PasetoClaimError(#[from] rusty_paseto::prelude::PasetoClaimError),

    /// Error that occurs while parsing a Paseto token.
    #[error("Paseto parser error: {0}")]
    PasetoParserError(#[from] rusty_paseto::generic::GenericParserError),

    /// Error that occurs while serializing or deserializing JSON data.
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Indicates an invalid Paseto token.
    #[error("Invalid token")]
    InvalidToken,
}

/// Represents a set of claims with ownership.
///
/// Owned claims are useful when working with deserialized Paseto tokens,
/// where string lifetimes are not known in advance.
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnedClaims<C> {
    /// The issuer of the token.
    issuer: String,
    /// The time at which the token was issued.
    issued_at: Option<DateTime<Utc>>,
    /// The expiration time of the token.
    expiration: DateTime<Utc>,
    /// The earliest time at which the token is valid.
    not_before: DateTime<Utc>,
    /// The subject of the token.
    subject: Option<String>,
    /// The intended audience of the token.
    audience: Option<String>,
    /// The unique identifier of the token.
    token_id: String,
    /// Any additional claims.
    #[serde(flatten)]
    other: Option<C>,
}

impl<C> OwnedClaims<C> {
    /// Returns the issuer of the token.
    pub fn issuer(&self) -> &str {
        self.issuer.as_ref()
    }

    /// Returns the time at which the token was issued.
    pub fn issued_at(&self) -> Option<DateTime<Utc>> {
        self.issued_at
    }

    /// Returns the expiration time of the token.
    pub fn expiration(&self) -> DateTime<Utc> {
        self.expiration
    }

    /// Returns the earliest time at which the token is valid.
    pub fn not_before(&self) -> DateTime<Utc> {
        self.not_before
    }

    /// Returns the subject of the token.
    pub fn subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    /// Returns the intended audience of the token.
    pub fn audience(&self) -> Option<&String> {
        self.audience.as_ref()
    }

    /// Returns the unique identifier of the token.
    pub fn token_id(&self) -> &str {
        self.token_id.as_ref()
    }

    /// Returns any additional claims.
    pub fn other(&self) -> Option<&C> {
        self.other.as_ref()
    }
}

/// Allows conversion from `DefaultClaims` to `OwnedClaims`.
impl<C> From<DefaultClaims> for OwnedClaims<C> {
    fn from(value: DefaultClaims) -> Self {
        OwnedClaims {
            issuer: value.issuer.to_string(),
            issued_at: value.issued_at,
            expiration: value.expiration,
            not_before: value.not_before,
            subject: value.subject,
            audience: value.audience,
            token_id: value.token_id,
            other: None,
        }
    }
}

/// Represents a set of default claims.
///
/// Default claims are useful when working with Paseto tokens that have
/// a known lifetime.
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultClaims {
    #[serde(rename = "iss")]
    issuer: String,

    #[serde(rename = "iat")]
    issued_at: Option<DateTime<Utc>>,

    #[serde(rename = "exp")]
    expiration: DateTime<Utc>,

    #[serde(rename = "nbf")]
    not_before: DateTime<Utc>,

    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    subject: Option<String>,

    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    audience: Option<String>,

    #[serde(rename = "jti")]
    token_id: String,

    #[serde(flatten)]
    other: Option<Value>,
}

impl DefaultClaims {
    pub fn builder<C, D>(issuer: C, expiration: DateTime<Utc>, token_id: D) -> DefaultClaimsBuilder
    where
        C: Into<String>,
        D: Into<String>,
    {
        DefaultClaimsBuilder::new(issuer.into(), expiration, token_id.into())
    }
}

pub struct DefaultClaimsBuilder {
    issuer: String,
    issued_at: Option<DateTime<Utc>>,
    expiration: DateTime<Utc>,
    not_before: DateTime<Utc>,
    subject: Option<String>,
    audience: Option<String>,
    token_id: String,
    other: Option<Value>,
}

impl DefaultClaimsBuilder {
    pub fn new(issuer: String, expiration: DateTime<Utc>, token_id: String) -> Self {
        let not_before = Utc::now();

        Self {
            issuer,
            issued_at: None,
            expiration,
            not_before,
            subject: None,
            audience: None,
            token_id,
            other: None,
        }
    }

    pub fn issued_at(mut self, issued_at: DateTime<Utc>) -> Self {
        self.issued_at = Some(issued_at);
        self
    }

    pub fn subject<C>(mut self, subject: C) -> Self
    where
        C: Into<String>,
    {
        self.subject = Some(subject.into());
        self
    }

    pub fn audience<C>(mut self, audience: C) -> Self
    where
        C: Into<String>,
    {
        self.audience = Some(audience.into());
        self
    }

    pub fn other<C>(mut self, other: C) -> Self
    where
        C: serde::Serialize,
    {
        serde_json::to_value(other)
            .map(|v| self.other = Some(v))
            .unwrap();
        self
    }

    pub fn not_before(mut self, not_before: DateTime<Utc>) -> Self {
        self.not_before = not_before;
        self
    }

    pub fn build(self) -> DefaultClaims {
        DefaultClaims {
            issuer: self.issuer,
            issued_at: self.issued_at,
            expiration: self.expiration,
            not_before: self.not_before,
            subject: self.subject,
            audience: self.audience,
            token_id: self.token_id,
            other: self.other,
        }
    }
}

pub type SymmetricKey = PasetoSymmetricKey<V4, Local>;

/// Generates a Paseto symmetric key from the provided key bytes.
///
/// # Arguments
///
/// * `key` - A byte slice representing the key.
///
/// # Returns
///
/// * A `PasetoSymmetricKey` instance.
pub fn generate_key(key: &[u8]) -> SymmetricKey {
    PasetoSymmetricKey::<V4, Local>::from(Key::from(key))
}

/// Encrypts a Paseto token with the provided claims and key.
///
/// # Arguments
///
/// * `default_claims` - A `DefaultClaims` instance containing the claims to be encrypted.
/// * `key` - A reference to a `PasetoSymmetricKey` instance.
///
/// # Returns
///
/// * A `Result` containing the encrypted token string, or an `Error`.
pub fn encrypt_token(
    default_claims: DefaultClaims,
    key: &PasetoSymmetricKey<V4, Local>,
) -> Result<String, Error> {
    let mut token: PasetoBuilder<V4, Local> = PasetoBuilder::<V4, Local>::default();
    let token = token
        .set_claim(ExpirationClaim::try_from(
            default_claims.expiration.to_rfc3339(),
        )?)
        .set_claim(NotBeforeClaim::try_from(
            default_claims.not_before.to_rfc3339(),
        )?)
        .set_claim(IssuerClaim::from(default_claims.issuer.as_str()))
        .set_claim(TokenIdentifierClaim::from(default_claims.token_id.as_str()));

    if let Some(subject) = &default_claims.subject {
        token.set_claim(SubjectClaim::from(subject.as_str()));
    }

    if let Some(audience) = &default_claims.audience {
        token.set_claim(AudienceClaim::from(audience.as_str()));
    }

    if let Some(other) = default_claims.other {
        if !Value::is_object(&other) {
            return Err(Error::InvalidToken);
        }

        let object: serde_json::Map<String, Value> = serde_json::from_value(other)?;
        for (key, value) in object {
            token.set_claim(CustomClaim::try_from((key.to_owned(), value))?);
        }
    }

    Ok(token.build(key)?)
}

/// Validates a Paseto token and extracts the claims.
///
/// # Arguments
///
/// * token - A string reference containing the Paseto token to validate.
/// * key - A reference to a PasetoSymmetricKey instance.
///
/// # Returns
///
/// * A Result containing an OwnedClaims<()> instance with the extracted claims, or an Error.
pub fn validate_token(
    token: &str,
    key: &PasetoSymmetricKey<V4, Local>,
) -> Result<OwnedClaims<()>, Error> {
    // Todo: extend function to allow checking of claims, validating audience, etc.

    let mut parser = PasetoParser::<V4, Local>::default();
    parser.check_claim(ExpirationClaim::default());
    parser.check_claim(NotBeforeClaim::default());

    let res = parser.parse(token, key)?;

    // Parse the claims
    let res = res.to_string();
    let claims: DefaultClaims = serde_json::from_str(&res)?;

    Ok(claims.into())
}

/// Validates a Paseto token and extracts the claims with a custom data type.
///
/// # Arguments
///
/// * token - A string reference containing the Paseto token to validate.
/// * key - A reference to a PasetoSymmetricKey instance.
///
/// # Type Parameters
///
/// * C - The custom data type for additional claims. Must implement DeserializeOwned.
///
/// # Returns
///
/// * A Result containing an OwnedClaims<C> instance with the extracted claims, or an Error.
pub fn validate_token_with<C>(
    token: &str,
    key: &PasetoSymmetricKey<V4, Local>,
) -> Result<OwnedClaims<C>, Error>
where
    C: DeserializeOwned,
{
    // Todo: extend function to allow checking of claims, validating audience, etc.

    let mut parser = PasetoParser::<V4, Local>::default();
    parser.check_claim(ExpirationClaim::default());
    parser.check_claim(NotBeforeClaim::default());

    let res = parser.parse(token, key)?;

    // Parse the claims
    let res = res.to_string();
    let mut claims: DefaultClaims = serde_json::from_str(&res)?;

    // Parse the other claims
    if let Some(other_claims) = claims.other.take() {
        let other_claims: C = serde_json::from_value(other_claims)?;
        let mut claims: OwnedClaims<C> = claims.into();
        claims.other = Some(other_claims);

        Ok(claims)
    } else {
        Err(Error::InvalidToken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key = generate_key(b"01234567890123456789012345678901");

        // test the key with a token
        let token = encrypt_token(
            DefaultClaims {
                issuer: "authcore".into(),
                issued_at: None,
                expiration: Utc::now() + chrono::Duration::days(1),
                not_before: Utc::now(),
                subject: None,
                audience: None,
                token_id: "user123".into(),

                other: None,
            },
            &key,
        );

        assert!(token.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_generate_bad_key() {
        let key = generate_key(b"-1");

        // test the key with a token
        let token = encrypt_token(
            DefaultClaims {
                issuer: "authcore".into(),
                issued_at: None,
                expiration: Utc::now() + chrono::Duration::days(1),
                not_before: Utc::now(),
                subject: None,
                audience: None,
                token_id: "user123".into(),

                other: None,
            },
            &key,
        );

        assert!(token.is_err());
    }

    #[test]
    fn test_encrypt_token() {
        let key = generate_key(b"01234567890123456789012345678901");

        // test the key with a token
        let token = encrypt_token(
            DefaultClaims {
                issuer: "authcore".into(),
                issued_at: None,
                expiration: Utc::now() + chrono::Duration::days(1),
                not_before: Utc::now(),
                subject: None,
                audience: None,
                token_id: "user123".into(),

                other: None,
            },
            &key,
        );

        assert!(token.is_ok());
    }
}
