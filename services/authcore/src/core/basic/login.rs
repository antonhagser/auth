use chrono::Duration;
use crypto::snowflake::Snowflake;
use thiserror::Error;

use crate::{
    core::token::{self, RefreshTokenError},
    models::{
        application::ReplicatedApplication,
        error::ModelError::{self},
        prisma,
        user::{User, UserToken, UserWith},
        PrismaClient,
    },
    state::AppState,
};

#[derive(Debug, Error)]
pub enum BasicLoginError {
    /// The email address format is invalid.
    #[error("invalid email address")]
    InvalidEmailAddressFormat,

    /// The application does not exist.
    #[error("application does not exist")]
    ApplicationDoesNotExist,

    /// The email address in combination with the password is invalid.
    #[error("wrong credentials")]
    WrongCredentials,

    /// The 2FA code is wrong.
    #[error("wrong 2FA code")]
    Wrong2FA,

    /// The account does not exist.
    #[error("account does not exist")]
    NotFound,

    #[error("database error")]
    QueryError(#[from] prisma_client_rust::QueryError),

    #[error("user needs further verification through 2FA")]
    NeedFurtherVerificationThrough2FA(Box<User>),

    #[error("unknown error")]
    Unknown,
}

/// Login with basic auth.
///
/// # Arguments
///
/// * `state` - The app state.
/// * `data` - The login data.
pub async fn with_basic_auth(
    prisma_client: &PrismaClient,
    email: String,
    password: String,
    application_id: Snowflake,
    ip_address: String,
) -> Result<User, BasicLoginError> {
    // Get application from database.
    let application = match ReplicatedApplication::get(prisma_client, application_id).await {
        Ok(app) => app,
        Err(ModelError::NotFound) => {
            return Err(BasicLoginError::ApplicationDoesNotExist);
        }
        _ => {
            return Err(BasicLoginError::Unknown);
        }
    };

    // Get user from database.
    let mut user = {
        match crate::models::user::User::find_by_email(
            prisma_client,
            email,
            application.application_id(),
            vec![UserWith::BasicAuth, UserWith::TOTP],
        )
        .await
        {
            Ok(user) => user,
            Err(ModelError::NotFound) => {
                return Err(BasicLoginError::NotFound);
            }
            _ => {
                return Err(BasicLoginError::Unknown);
            }
        }
    };

    // Check if the user has a password.
    let auth = user.basic_auth(None).await;

    if auth.is_none() {
        return Err(BasicLoginError::WrongCredentials);
    }

    // Check if the password is correct.
    if crypto::password::verify_password(&password, auth.as_ref().unwrap().password_hash()).is_err()
    {
        return Err(BasicLoginError::WrongCredentials);
    }

    // If user does not have 2FA enabled, return the user.
    if user.totp().is_some() {
        return Err(BasicLoginError::NeedFurtherVerificationThrough2FA(
            Box::new(user),
        ));
    }

    prisma_client
        .user()
        .update(
            prisma::user::id::equals(user.id().to_id_signed()),
            vec![
                prisma::user::last_login_at::set(Some(chrono::Utc::now().into())),
                prisma::user::last_login_ip::set(Some(ip_address)),
            ],
        )
        .exec()
        .await?;

    Ok(user)
}

pub async fn create_refresh_and_access_token(
    state: &AppState,
    prisma_client: &PrismaClient,
    user: &User,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(UserToken, String), RefreshTokenError> {
    // Generate a new user refresh token
    let refresh_token = token::new_refresh_token(
        state,
        prisma_client,
        user.id(),
        chrono::Utc::now() + Duration::days(30),
        ip_address,
        user_agent,
    )
    .await?;

    // Generate access token
    let access_token = token::new_access_token(
        state,
        user.id(),
        chrono::Utc::now() + Duration::hours(1),
        refresh_token.id(),
    )?;

    Ok((refresh_token, access_token))
}
