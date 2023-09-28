use chrono::Utc;
use crypto::{
    snowflake::Snowflake,
    tokens::paseto::{self, DefaultClaims, OwnedClaims},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims<C> {
    token_id: Snowflake,

    #[serde(flatten)]
    generic: Option<C>,
}

impl<C> TokenClaims<C>
where
    C: Serialize + DeserializeOwned,
{
    pub fn token_id(&self) -> Snowflake {
        self.token_id
    }

    pub fn take_generic(&mut self) -> Option<C> {
        self.generic.take()
    }
}

pub fn generate_generic_token<C>(
    state: &AppState,
    token_id: Snowflake,
    user_id: Snowflake,
    expiration: chrono::DateTime<Utc>,
    generic: C,
) -> Result<String, paseto::Error>
where
    C: Serialize + DeserializeOwned,
{
    let default_claims = DefaultClaims::builder("AuthCore", expiration, token_id)
        .subject(user_id)
        .not_before(Utc::now())
        .other(TokenClaims {
            token_id,
            generic: Some(generic),
        })
        .build();

    paseto::encrypt_token(default_claims, state.paseto_key())
}

pub fn verify_generic_token<C>(
    state: &AppState,
    token: &str,
) -> Result<OwnedClaims<TokenClaims<C>>, paseto::Error>
where
    C: Serialize + DeserializeOwned,
{
    let res: OwnedClaims<TokenClaims<C>> = paseto::validate_token(token, state.paseto_key())?;

    Ok(res)
}
