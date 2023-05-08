use chrono::{DateTime, Utc};
use rusty_paseto::prelude::{
    AudienceClaim, CustomClaim, ExpirationClaim, IssuerClaim, Key, Local, NotBeforeClaim,
    PasetoBuilder, PasetoParser, PasetoSymmetricKey, SubjectClaim, TokenIdentifierClaim, V4,
};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Paseto builder error: {0}")]
    PasetoBuilderError(#[from] rusty_paseto::generic::GenericBuilderError),

    #[error("Paseto parser error: {0}")]
    PasetoParserError(#[from] rusty_paseto::generic::GenericParserError),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnedClaims<C> {
    pub issuer: String,
    pub issued_at: Option<DateTime<Utc>>,
    pub expiration: DateTime<Utc>,
    pub not_before: DateTime<Utc>,
    pub subject: Option<String>,
    pub audience: Option<String>,
    pub token_id: String,
    pub other: Option<C>,
}

impl<'a, C> From<DefaultClaims<'a>> for OwnedClaims<C> {
    fn from(value: DefaultClaims<'a>) -> Self {
        OwnedClaims {
            issuer: value.issuer.to_string(),
            issued_at: value.issued_at,
            expiration: value.expiration,
            not_before: value.not_before,
            subject: value.subject.map(|s| s.to_string()),
            audience: value.audience.map(|s| s.to_string()),
            token_id: value.token_id.to_string(),
            other: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultClaims<'a> {
    #[serde(rename = "iss")]
    issuer: &'a str,

    #[serde(rename = "iat")]
    issued_at: Option<DateTime<Utc>>,

    #[serde(rename = "exp")]
    expiration: DateTime<Utc>,

    #[serde(rename = "nbf")]
    not_before: DateTime<Utc>,

    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    subject: Option<&'a str>,

    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    audience: Option<&'a str>,

    #[serde(rename = "jti")]
    token_id: &'a str,

    #[serde(flatten)]
    other: Option<Value>,
}

impl<'a> DefaultClaims<'a> {
    pub fn new(
        issuer: &'a str,
        expiration: DateTime<Utc>,
        not_before: DateTime<Utc>,
        token_id: &'a str,
        other: Option<Value>,
    ) -> Self {
        Self {
            issuer,
            issued_at: None,
            expiration,
            not_before,
            subject: None,
            audience: None,
            token_id,
            other,
        }
    }

    pub fn set_subject(&mut self, subject: &'a str) {
        self.subject = Some(subject);
    }

    pub fn set_audience(&mut self, audience: &'a str) {
        self.audience = Some(audience);
    }

    pub fn issuer(&self) -> &str {
        self.issuer
    }

    pub fn issued_at(&self) -> Option<DateTime<Utc>> {
        self.issued_at
    }

    pub fn not_before(&self) -> DateTime<Utc> {
        self.not_before
    }

    pub fn expiration(&self) -> DateTime<Utc> {
        self.expiration
    }

    pub fn subject(&self) -> Option<&str> {
        self.subject
    }

    pub fn audience(&self) -> Option<&str> {
        self.audience
    }

    pub fn token_id(&self) -> &str {
        self.token_id
    }

    pub fn other(&self) -> Option<&Value> {
        self.other.as_ref()
    }
}

pub fn generate_key(key: &[u8]) -> PasetoSymmetricKey<V4, Local> {
    PasetoSymmetricKey::<V4, Local>::from(Key::from(key))
}

pub fn encrypt_token(
    default_claims: DefaultClaims,
    key: &PasetoSymmetricKey<V4, Local>,
) -> Result<String, Error> {
    let mut token: PasetoBuilder<V4, Local> = PasetoBuilder::<V4, Local>::default();
    let token = token
        .set_claim(ExpirationClaim::try_from(default_claims.expiration.to_rfc3339()).unwrap())
        .set_claim(NotBeforeClaim::try_from(default_claims.not_before.to_rfc3339()).unwrap())
        .set_claim(IssuerClaim::from(default_claims.issuer))
        .set_claim(TokenIdentifierClaim::from(default_claims.token_id));

    if let Some(subject) = default_claims.subject {
        token.set_claim(SubjectClaim::from(subject));
    }

    if let Some(audience) = default_claims.audience {
        token.set_claim(AudienceClaim::from(audience));
    }

    if let Some(other) = default_claims.other {
        if !Value::is_object(&other) {
            return Err(Error::InvalidToken);
        }

        let object: serde_json::Map<String, Value> = serde_json::from_value(other)?;
        for (key, value) in object {
            token.set_claim(CustomClaim::try_from((key.to_owned(), value)).unwrap());
        }
    }

    Ok(token.build(key)?)
}

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
    let claims: DefaultClaims = serde_json::from_str(&res).unwrap();

    Ok(claims.into())
}

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
    let mut claims: DefaultClaims = serde_json::from_str(&res).unwrap();

    // Parse the other claims
    if let Some(other_claims) = claims.other.take() {
        let other_claims: C = serde_json::from_value(other_claims).unwrap();
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
                issuer: "authcore",
                issued_at: None,
                expiration: Utc::now() + chrono::Duration::days(1),
                not_before: Utc::now(),
                subject: None,
                audience: None,
                token_id: "user123",

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
                issuer: "authcore",
                issued_at: None,
                expiration: Utc::now() + chrono::Duration::days(1),
                not_before: Utc::now(),
                subject: None,
                audience: None,
                token_id: "user123",

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
                issuer: "authcore",
                issued_at: None,
                expiration: Utc::now() + chrono::Duration::days(1),
                not_before: Utc::now(),
                subject: None,
                audience: None,
                token_id: "user123",

                other: None,
            },
            &key,
        );

        assert!(token.is_ok());
    }
}
