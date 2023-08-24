use crypto::snowflake::Snowflake;
use thiserror::Error;

use crate::{
    models::{
        application::ReplicatedApplication,
        error::ModelError::{self, NotFound},
        user::{User, UserWith},
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

pub struct BasicLoginData {
    pub email: String,

    pub password: String,

    pub application_id: Snowflake,

    // Todo: future idea, implement device id to track devices.
    // pub device_id: Option<Snowflake>,
    pub ip_address: String,

    pub user_agent: String,

    pub totp_code: Option<String>,
}

/// Login with basic auth.
///
/// # Arguments
///
/// * `state` - The app state.
/// * `data` - The login data.
pub async fn with_basic_auth(
    state: &AppState,
    data: BasicLoginData,
) -> Result<User, BasicLoginError> {
    // Get application from database.
    let application =
        match ReplicatedApplication::find_by_id(state.prisma(), data.application_id).await {
            Ok(app) => app,
            Err(NotFound) => {
                return Err(BasicLoginError::ApplicationDoesNotExist);
            }
            _ => {
                return Err(BasicLoginError::Unknown);
            }
        };

    // Get user from database.
    let mut user = {
        match crate::models::user::User::find_by_email(
            state.prisma(),
            data.email,
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
    if crypto::password::verify_password(&data.password, auth.as_ref().unwrap().password_hash())
        .is_err()
    {
        return Err(BasicLoginError::WrongCredentials);
    }

    // Check if user has 2FA through TOTP enabled.
    let totp = user.totp();

    // If user does not have 2FA enabled, return the user.
    if totp.is_none() {
        return Ok(user);
    }

    // If user has 2FA enabled, check if user has 2FA through TOTP enabled.
    let totp = totp.unwrap();

    // If the user provided a TOTP code, check if it is correct.
    if let Some(totp_code) = data.totp_code {
        if !totp.verify(totp_code) {
            return Err(BasicLoginError::Wrong2FA);
        }
    } else {
        // If the user did not provide a TOTP code, return the user id.
        return Err(BasicLoginError::NeedFurtherVerificationThrough2FA(
            Box::new(user),
        ));
    }

    // Check if user has 2FA through U2F enabled.

    Ok(user)
}
