use chrono::Utc;
use crypto::snowflake::Snowflake;
use serde::{Deserialize, Serialize};

use crate::{
    core::token::{generate_generic_token, verify_generic_token},
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailVerificationTokenData {
    pub user_id: Snowflake,
    pub email_id: Snowflake,
    pub application_id: Snowflake,
}

#[derive(Debug)]
pub struct EmailVerificationToken {
    pub token: String,
    pub token_id: Snowflake,
    pub data: EmailVerificationTokenData,
}

impl EmailVerificationToken {
    pub fn new(
        state: &AppState,
        token_id: Snowflake,
        user_id: Snowflake,
        email_id: Snowflake,
        application_id: Snowflake,
        expires_at: chrono::DateTime<Utc>,
    ) -> Result<Self, anyhow::Error> {
        // If method is email generate a token
        let token = generate_generic_token(
            state,
            token_id,
            user_id,
            expires_at,
            EmailVerificationTokenData {
                user_id,
                email_id,
                application_id,
            },
        )?;

        Ok(Self {
            token,
            token_id,
            data: EmailVerificationTokenData {
                user_id,
                email_id,
                application_id,
            },
        })
    }

    pub fn from_raw(state: &AppState, raw_token: &str) -> Result<Self, anyhow::Error> {
        let mut token = verify_generic_token(state, raw_token)?;
        let claims = token.other_mut();

        let mut claims = claims
            .take()
            .ok_or_else(|| anyhow::anyhow!("invalid token"))?;

        let generics = claims.take_generic();

        Ok(Self {
            token: raw_token.into(),
            token_id: claims.token_id(),
            data: generics.unwrap(),
        })
    }

    pub fn data(&self) -> &EmailVerificationTokenData {
        &self.data
    }

    pub fn token(&self) -> &str {
        self.token.as_ref()
    }
}
