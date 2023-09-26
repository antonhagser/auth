use chrono::{DateTime, Utc};
use crypto::{
    snowflake::Snowflake,
    tokens::paseto::{self, DefaultClaims, OwnedClaims},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::state::AppState;

#[derive(Debug, Error)]
pub enum AccessTokenError {
    #[error("internal paseto error")]
    PasetoError(#[from] paseto::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    refresh_token_id: Snowflake,
}

pub fn new_access_token(
    state: &AppState,
    user_id: Snowflake,
    expiration: DateTime<Utc>,
    refresh_token_id: Snowflake,
) -> Result<String, paseto::Error> {
    // TODO: A client application should be able to define a custom paseto token layout, also be able to switch to using JWTs
    let default_claims = DefaultClaims::builder(
        "AuthCore",
        expiration,
        state.id_generator().next_snowflake().unwrap(),
    )
    .subject(user_id)
    .audience("AuthCore")
    .not_before(Utc::now())
    .other(AccessTokenClaims { refresh_token_id })
    .build();

    paseto::encrypt_token(default_claims, state.paseto_key())
}

pub fn verify_access_token(
    state: &AppState,
    token: &str,
) -> Result<OwnedClaims<()>, AccessTokenError> {
    let claims = paseto::validate_token(token, state.paseto_key())?;

    Ok(claims)
}
