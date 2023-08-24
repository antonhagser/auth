use chrono::{DateTime, Utc};
use crypto::{
    snowflake::{Snowflake, SnowflakeGenerator},
    tokens::paseto::{self, DefaultClaims},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{error::ModelError, prisma::UserTokenType, user::UserToken, PrismaClient},
    state::AppState,
};

pub async fn new_refresh_token(
    client: &PrismaClient,
    id_generator: &SnowflakeGenerator,
    user_id: Snowflake,
    expires_at: DateTime<Utc>,
) -> Result<UserToken, ModelError> {
    let token = crypto::tokens::string::random_token();

    UserToken::builder(
        id_generator,
        user_id,
        UserTokenType::Refresh,
        token,
        expires_at,
    )
    .build(client)
    .await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {}

pub fn new_access_token(
    state: &AppState,
    subject: Snowflake,
    expiration: DateTime<Utc>,
    refresh_token_id: Snowflake,
) -> Result<String, paseto::Error> {
    // TODO: A client application should be able to define a custom paseto token layout, also be able to switch to using JWTs
    let default_claims = DefaultClaims::builder("AuthCore", expiration, refresh_token_id)
        .subject(subject)
        .build();

    paseto::encrypt_token(default_claims, state.paseto_key())
}
