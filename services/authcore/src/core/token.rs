use chrono::{DateTime, Utc};
use crypto::{
    snowflake::Snowflake,
    tokens::paseto::{self, DefaultClaims, OwnedClaims},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    models::{error::ModelError, prisma::UserTokenType, user::UserToken, PrismaClient},
    state::AppState,
};

#[derive(Debug, Error)]
pub enum RefreshTokenError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token revoked")]
    TokenRevoked,

    #[error("internal paseto error")]
    PasetoError(#[from] paseto::Error),

    #[error("database error")]
    InternalDatabaseError(#[from] ModelError),

    #[error("database error")]
    QueryError(#[from] prisma_client_rust::QueryError),
}

pub async fn new_refresh_token(
    state: &AppState,
    prisma_client: &PrismaClient,
    user_id: Snowflake,
    expires_at: DateTime<Utc>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<UserToken, RefreshTokenError> {
    let token_id = state.id_generator().next_snowflake().unwrap();

    let default_claims = DefaultClaims::builder("AuthCore", expires_at, token_id)
        .subject(user_id)
        .build();

    let token = paseto::encrypt_token(default_claims, state.paseto_key())?;

    let res = UserToken::builder(token_id, user_id, UserTokenType::Refresh, token, expires_at)
        .ip_address(ip_address)
        .user_agent(user_agent)
        .build(prisma_client)
        .await?;

    Ok(res)
}

pub async fn verify_refresh_token(
    state: &AppState,
    prisma_client: &PrismaClient,
    token: &str,
) -> Result<UserToken, RefreshTokenError> {
    // Parse the token
    let claims = paseto::validate_token(token, state.paseto_key())?;

    // Validate it against the database
    let token = UserToken::get(
        prisma_client,
        claims.subject().unwrap().to_owned().try_into().unwrap(),
        claims.token_id().try_into().unwrap(),
        UserTokenType::Refresh,
    )
    .await?;

    if token.expires_at() < Utc::now() {
        return Err(RefreshTokenError::TokenExpired);
    }

    Ok(token)
}

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
