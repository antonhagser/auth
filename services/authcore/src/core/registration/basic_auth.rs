use crypto::snowflake::Snowflake;
use thiserror::Error;
use tracing::info;

use crate::{
    models::{error::ModelError, user::User},
    state::AppState,
};

pub mod password;
pub mod username;

#[derive(Debug, Error)]
pub enum BasicRegistrationError {
    #[error("invalid email address")]
    InvalidEmailAddress,
    #[error("email address already exists")]
    EmailAddressAlreadyExists,
    #[error("invalid password")]
    InvalidPassword(Vec<password::PasswordValidationError>),
    #[error("invalid username")]
    InvalidUsername,
    #[error("username already exists")]
    UsernameAlreadyExists,

    #[error("application does not exist")]
    ApplicationDoesNotExist,

    #[error("database error")]
    QueryError(#[from] prisma_client_rust::QueryError),

    #[error("unknown error")]
    Unknown,
}

pub struct BasicRegistrationData {
    pub email: String,
    pub username: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub password: String,

    pub application_id: Snowflake,
}

/// Register a new user with basic auth.
///
/// # Arguments
///
/// * `state` - The app state.
/// * `data` - The registration data.
pub async fn with_basic_auth(
    state: &AppState,
    mut data: BasicRegistrationData,
) -> Result<(), BasicRegistrationError> {
    // validate email
    if !crypto::input::email::validate_email(&data.email) {
        return Err(BasicRegistrationError::InvalidEmailAddress);
    }

    // check if email already exists
    let res = User::exists(
        state.prisma(),
        crate::models::user::ExistsOr::Email(&data.email),
    )
    .await;
    match res {
        Err(ModelError::DatabaseError(e)) => return Err(BasicRegistrationError::QueryError(e)),
        Ok(true) => return Err(BasicRegistrationError::EmailAddressAlreadyExists),
        _ => (),
    }

    // validate password
    let mut user_input = Vec::new();
    user_input.push(data.email.clone());
    if let Some(username) = &data.username {
        user_input.push(username.clone());
    }

    if let Some(first_name) = &data.first_name {
        user_input.push(first_name.clone());
    }

    if let Some(last_name) = &data.last_name {
        user_input.push(last_name.clone());
    }

    if let Err(e) = password::validate_password(
        state.config().default_password_requirements,
        &data.password,
        user_input,
    ) {
        return Err(BasicRegistrationError::InvalidPassword(e));
    }

    let mut user = User::builder(
        state.id_generator(),
        state.prisma(),
        data.application_id,
        data.email,
    );

    // if username is provided, validate it and add it to builder (if not already taken)
    if let Some(username) = data.username.take() {
        let _ = username::validate_username(&username);

        // check if username already exists
        let res = User::exists(
            state.prisma(),
            crate::models::user::ExistsOr::Username(&username),
        )
        .await;
        match res {
            Err(ModelError::DatabaseError(e)) => return Err(BasicRegistrationError::QueryError(e)),
            Ok(true) => return Err(BasicRegistrationError::UsernameAlreadyExists),
            _ => (),
        }

        user.username(username);
    }

    // if first name is provided, add it to builder
    if let Some(first_name) = data.first_name.take() {
        user.first_name(first_name);
    }

    // if last name is provided, add it to builder
    if let Some(last_name) = data.last_name.take() {
        user.last_name(last_name);
    }

    // add password to builder
    let password_hash = crypto::password::hash_and_salt_password(&data.password);

    // TODO: handle error
    if password_hash.is_err() {
        return Err(BasicRegistrationError::Unknown);
    }

    // Set the password hash
    user.basic_auth(password_hash.unwrap());

    // create user
    let user = user.build().await;
    let user = match user {
        Ok(user) => user,
        Err(ModelError::DatabaseError(e)) => {
            if e.is_prisma_error::<prisma_client_rust::prisma_errors::query_engine::RecordRequiredButNotFound>() {
                return Err(BasicRegistrationError::ApplicationDoesNotExist);
            }

            return Err(BasicRegistrationError::QueryError(e));
        }
        _ => return Err(BasicRegistrationError::Unknown),
    };

    info!("user created: {:#?}", user);

    Ok(())
}
