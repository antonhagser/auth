use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use chrono::{DateTime, Utc};
use crypto::{
    snowflake::Snowflake,
    tokens::paseto::{self, DefaultClaims},
};
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

/// Create a new refresh token.
/// # Arguments
/// * `state` - The app state.
/// * `user_id` - The user ID.
/// * `expires_at` - The expiration date.
/// * `ip_address` - The IP address.
/// * `user_agent` - The user agent.
///
/// # Returns
/// * `Ok(UserToken)` - The user token.
/// * `Err(RefreshTokenError)` - The error.
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

/// Verify a refresh token.
/// # Arguments
/// * `state` - The app state.
/// * `token` - The refresh token.
/// * `application_id` - The application ID.
///
/// # Returns
/// * `Ok(UserToken)` - The user token.
/// * `Err(RefreshTokenError)` - The error.
pub async fn verify_refresh_token(
    state: &AppState,
    prisma_client: &PrismaClient,
    token: &str,
) -> Result<UserToken, RefreshTokenError> {
    // Parse the token
    let claims = paseto::validate_token::<()>(token, state.paseto_key())?;

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

pub fn create_refresh_cookie<'a>(
    token: String,
    expire: DateTime<Utc>,
    application_id: Snowflake,
) -> Cookie<'a> {
    let expiration_time = time::OffsetDateTime::from_unix_timestamp(expire.timestamp()).unwrap();
    let refresh_cookie_name = build_refresh_cookie_name(application_id);

    Cookie::build(refresh_cookie_name, token)
        .domain("localhost")
        .secure(false) // TODO: set to true
        .http_only(true)
        .expires(expiration_time)
        .path("/")
        .same_site(SameSite::Strict)
        .finish()
}

pub fn get_refresh_cookie(jar: &CookieJar, application_id: Snowflake) -> Option<&Cookie<'_>> {
    let refresh_cookie_name = build_refresh_cookie_name(application_id);
    jar.get(&refresh_cookie_name)
}

fn build_refresh_cookie_name(application_id: Snowflake) -> String {
    let application_id = crypto::totp::BASE32_NOPAD
        .encode(application_id.to_string().as_bytes())
        .to_lowercase();
    let refresh_cookie_name = format!("refresh_{}", application_id);

    refresh_cookie_name
}
