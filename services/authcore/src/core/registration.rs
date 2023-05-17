use thiserror::Error;

use crate::{
    models::{error::ModelError, user::EmailAddress},
    state::AppState,
};

#[derive(Debug, Error)]
pub enum RegistrationError {
    #[error("invalid email address")]
    InvalidEmailAddress,
    #[error("email address already exists")]
    EmailAddressAlreadyExists,
    #[error("invalid password")]
    InvalidPassword,
    #[error("invalid phone number")]
    InvalidPhoneNumber,
    #[error("phone number already exists")]
    PhoneNumberAlreadyExists,
    #[error("invalid username")]
    InvalidUsername,
    #[error("username already exists")]
    UsernameAlreadyExists,

    #[error("database error")]
    QueryError(#[from] prisma_client_rust::QueryError),
}

pub struct RegistrationData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub phone_number: String,

    pub application_id: u64,
}

/// Register a new user with basic auth.
///
/// # Arguments
///
/// * `state` - The app state.
/// * `data` - The registration data.
pub async fn with_basic_auth(
    state: &AppState,
    data: RegistrationData,
) -> Result<(), RegistrationError> {
    let email = data.email;

    // validate email
    if !crypto::input::email::validate_email(&email) {
        return Err(RegistrationError::InvalidEmailAddress);
    }

    // check if email already exists
    let res = EmailAddress::exists(state.prisma(), &email).await;
    match res {
        Err(ModelError::DatabaseError(e)) => return Err(RegistrationError::QueryError(e)),
        Ok(true) => return Err(RegistrationError::EmailAddressAlreadyExists),
        _ => (),
    }

    // check if username already exists

    Ok(())
}
