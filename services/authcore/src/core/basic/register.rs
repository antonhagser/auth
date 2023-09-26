use crypto::{input::password, snowflake::Snowflake};
use thiserror::Error;
use tracing::info;

use crate::{
    models::{error::ModelError, user::User},
    state::AppState,
};

#[derive(Debug, Error)]
pub enum BasicRegistrationError {
    #[error("invalid email address")]
    EmailFormat,

    #[error("invalid password")]
    PasswordFormat(Vec<password::PasswordValidationError>),

    #[error("email address already exists")]
    AlreadyExists,

    #[error("application does not exist")]
    ApplicationDoesNotExist,

    #[error("internal server error")]
    InternalServerError,
}

pub struct BasicRegistrationData {
    pub email: String,

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
) -> Result<User, BasicRegistrationError> {
    // validate email
    if !crypto::input::email::validate_email(&data.email) {
        return Err(BasicRegistrationError::EmailFormat);
    }

    // check if email already exists
    match User::find_by_email(state.prisma(), &data.email, data.application_id, vec![]).await {
        Ok(_) => return Err(BasicRegistrationError::AlreadyExists),
        Err(ModelError::NotFound) => (),
        _ => return Err(BasicRegistrationError::InternalServerError),
    }

    // validate password
    let mut user_inputs = Vec::new();
    user_inputs.push(data.email.clone());

    if let Some(first_name) = &data.first_name {
        user_inputs.push(first_name.clone());
    }

    if let Some(last_name) = &data.last_name {
        user_inputs.push(last_name.clone());
    }

    // Get the password requirements from the application
    // TODO: Introduce caching? (big problem with cache invalidation, maybe we're fine with a bit of a delay?)
    let mut application =
        match crate::models::application::ReplicatedApplication::find_by_id_with_config(
            state.prisma(),
            data.application_id,
        )
        .await
        {
            Ok(application) => application,
            Err(_) => return Err(BasicRegistrationError::ApplicationDoesNotExist),
        };

    // If password requirements config is not found, use the default config
    let password_requirements = application
        .basic_auth_config(state.prisma())
        .await
        .as_password_requirements_config();

    // Validate the password
    let user_inputs: Vec<&str> = user_inputs.iter().map(|s| s.as_str()).collect();
    if let Err(e) =
        password::validate_password(&data.password, &user_inputs, true, password_requirements)
    {
        return Err(BasicRegistrationError::PasswordFormat(e));
    }

    // create user builder
    let mut user = User::builder(
        state.id_generator(),
        state.prisma(),
        data.application_id,
        data.email,
    );

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
    if password_hash.is_err() {
        return Err(BasicRegistrationError::InternalServerError);
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

            return Err(BasicRegistrationError::InternalServerError);
        }
        _ => return Err(BasicRegistrationError::InternalServerError),
    };

    info!("user created: {:#?}", user);

    Ok(user)
}
