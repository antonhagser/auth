use chrono::Utc;
use crypto::{
    snowflake::Snowflake,
    tokens::paseto::{self, DefaultClaims},
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    token_id: Snowflake,
}

pub fn generate_generic_token(
    state: &AppState,
    token_id: Snowflake,
    user_id: Snowflake,
    expiration: chrono::DateTime<Utc>,
) -> Result<String, paseto::Error> {
    let default_claims = DefaultClaims::builder("AuthCore", expiration, token_id)
        .subject(user_id)
        .not_before(Utc::now())
        .other(TokenClaims { token_id })
        .build();

    paseto::encrypt_token(default_claims, state.paseto_key())
}

pub fn verify_generic_token(state: &AppState, token: &str) -> Result<(), paseto::Error> {
    let _ = paseto::validate_token(token, state.paseto_key())?;

    Ok(())
}
