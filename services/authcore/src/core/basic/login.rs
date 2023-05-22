use crypto::snowflake::Snowflake;
use thiserror::Error;

use crate::{
    models::{
        application::ReplicatedApplication,
        error::ModelError::{self, RecordNotFound},
        user::{User, UserWith},
    },
    state::AppState,
};

#[derive(Debug, Error)]
pub enum BasicLoginError {
    /// The email address format is invalid.
    #[error("invalid email address")]
    InvalidEmailAddressFormat,
    /// The username format is invalid.
    #[error("invalid username")]
    InvalidUsernameFormat,

    /// The application does not exist.
    #[error("application does not exist")]
    ApplicationDoesNotExist,

    /// The email address or username in combination with the password is invalid.
    #[error("wrong credentials")]
    WrongCredentials,

    /// The account does not exist.
    #[error("account does not exist")]
    NotFound,

    #[error("database error")]
    QueryError(#[from] prisma_client_rust::QueryError),

    #[error("unknown error")]
    Unknown,
}

pub struct BasicLoginData {
    pub email_or_username: crate::models::user::ExistsOr<String>,

    pub password: String,

    pub application_id: Snowflake,
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
            Err(RecordNotFound) => {
                return Err(BasicLoginError::ApplicationDoesNotExist);
            }
            _ => {
                return Err(BasicLoginError::Unknown);
            }
        };

    // Do not validate password due to the possibility of password requirements changing.

    // Get user from database.
    let user = match data.email_or_username {
        crate::models::user::ExistsOr::Email(email) => {
            match crate::models::user::User::find_by_email(
                state.prisma(),
                email,
                application.application_id(),
                vec![UserWith::BasicAuth],
            )
            .await
            {
                Ok(user) => user,
                Err(ModelError::RecordNotFound) => {
                    return Err(BasicLoginError::NotFound);
                }
                _ => {
                    return Err(BasicLoginError::Unknown);
                }
            }
        }
        crate::models::user::ExistsOr::Username(username) => {
            match crate::models::user::User::find_by_username(
                state.prisma(),
                username,
                application.application_id(),
                vec![UserWith::BasicAuth],
            )
            .await
            {
                Ok(user) => user,
                Err(ModelError::RecordNotFound) => {
                    return Err(BasicLoginError::NotFound);
                }
                _ => {
                    return Err(BasicLoginError::Unknown);
                }
            }
        }
    };

    // Check if the user has a password.
    if user.basic_auth().is_not_loaded() {
        return Err(BasicLoginError::WrongCredentials);
    }

    // Check if the password is correct.
    if crypto::password::verify_password(
        &data.password,
        user.basic_auth().as_ref().unwrap().password_hash(),
    )
    .is_err()
    {
        return Err(BasicLoginError::WrongCredentials);
    }

    Ok(user)
}
