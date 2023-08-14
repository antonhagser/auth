use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};
use prisma_client_rust::QueryError;

use self::email_address::EmailAddressBuilder;

use super::{
    error::ModelError,
    prisma::{self, basic_auth, user::Data, PrismaClient},
    ModelValue,
};

pub use email_address::EmailAddress;
pub use external_user::ExternalUser;
pub use token::UserToken;

mod email_address;
mod external_user;
mod token;

#[derive(Debug, Clone)]
pub struct User {
    id: Snowflake,

    // Todo: Replace Option with a new type which implements a enum None (exists but not loaded), NotSet (not set), Some (set to a value)
    first_name: Option<String>,
    last_name: Option<String>,

    email_address: ModelValue<EmailAddress>,

    external_users: Vec<ExternalUser>,

    password_enabled: bool,
    basic_auth: ModelValue<BasicAuth>,

    last_login_at: Option<DateTime<Utc>>,
    last_login_ip: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    sessions: Vec<Session>,
    user_metadata: Vec<UserMetadata>,

    application_id: Snowflake,
}

pub enum UserWith {
    EmailAddress,
    BasicAuth,
}

impl User {
    pub async fn find_by_email<C>(
        client: &PrismaClient,
        email: C,
        application_id: Snowflake,
        user_with: Vec<UserWith>,
    ) -> Result<User, ModelError>
    where
        C: Into<String>,
    {
        let mut user_fetch = super::prisma::email_address::user::fetch();
        for with in user_with {
            user_fetch = match with {
                UserWith::EmailAddress => user_fetch,
                UserWith::BasicAuth => user_fetch.with(super::prisma::user::basic_auth::fetch()),
            };
        }

        let email_address = client
            .email_address()
            .find_first(vec![
                super::prisma::email_address::email_address::equals(email.into()),
                super::prisma::email_address::replicated_application_id::equals(
                    application_id.to_id_signed(),
                ),
            ])
            .with(user_fetch)
            .exec()
            .await
            .map_err(ModelError::DatabaseError)?;

        match email_address {
            Some(mut email_address) => {
                let user = email_address
                    .user
                    .take()
                    .ok_or(ModelError::RecordNotFound)?;
                let mut user: User = (*user).into();

                let email_address: EmailAddress = email_address.into();
                user.email_address = ModelValue::Loaded(email_address);
                Ok(user)
            }
            None => Err(ModelError::RecordNotFound),
        }
    }

    pub fn builder<'a>(
        id_generator: &'a SnowflakeGenerator,
        client: &'a PrismaClient,
        application_id: Snowflake,
        email: String,
    ) -> UserBuilder<'a> {
        let user_id = id_generator.next_snowflake().unwrap();
        let email_builder = EmailAddress::builder(id_generator, user_id).email_address(email);

        UserBuilder {
            id_generator,
            client,

            first_name: None,
            last_name: None,
            email_builder,
            basic_auth_builder: None,

            id: user_id,
            application_id,

            user_metadata: vec![],
        }
    }

    pub fn id(&self) -> Snowflake {
        self.id
    }

    pub fn first_name(&self) -> Option<&String> {
        self.first_name.as_ref()
    }

    pub fn last_name(&self) -> Option<&String> {
        self.last_name.as_ref()
    }

    pub fn email_address(&self) -> ModelValue<&EmailAddress> {
        self.email_address.as_ref()
    }

    pub fn external_users(&self) -> &[ExternalUser] {
        self.external_users.as_ref()
    }

    pub fn password_enabled(&self) -> bool {
        self.password_enabled
    }

    pub fn basic_auth(&self) -> ModelValue<&BasicAuth> {
        self.basic_auth.as_ref()
    }

    pub fn last_login_at(&self) -> Option<DateTime<Utc>> {
        self.last_login_at
    }

    pub fn last_login_ip(&self) -> Option<&String> {
        self.last_login_ip.as_ref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn sessions(&self) -> &[Session] {
        self.sessions.as_ref()
    }

    pub fn user_metadata(&self) -> &[UserMetadata] {
        self.user_metadata.as_ref()
    }

    pub fn application_id(&self) -> Snowflake {
        self.application_id
    }
}

impl From<Data> for User {
    fn from(value: Data) -> Self {
        // Check if email address is loaded
        let email_address = if let Some(email_address) = value.email_address {
            if let Some(email_address) = email_address {
                ModelValue::Loaded(EmailAddress::from(*email_address))
            } else {
                ModelValue::NotSet
            }
        } else {
            ModelValue::NotLoaded
        };

        // Check if basic auth is loaded
        let basic_auth = if let Some(basic_auth) = value.basic_auth {
            if let Some(basic_auth) = basic_auth {
                ModelValue::Loaded(BasicAuth::from(*basic_auth))
            } else {
                ModelValue::NotSet
            }
        } else {
            ModelValue::NotLoaded
        };

        User {
            id: value.id.try_into().unwrap(),
            first_name: value.first_name,
            last_name: value.last_name,
            email_address,
            // Todo: Implement ModelValue for external users
            external_users: vec![],
            password_enabled: value.password_enabled,
            basic_auth,
            last_login_at: value.last_login_at.map(|v| v.into()),
            last_login_ip: value.last_login_ip,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            // Todo: Implement ModelValue
            sessions: vec![],
            // Todo: Implement ModelValue
            user_metadata: vec![],
            application_id: value.replicated_application_id.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicAuth {
    user_id: Snowflake,

    password_hash: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BasicAuth {
    pub fn builder(user_id: Snowflake, password_hash: String) -> BasicAuthBuilder {
        BasicAuthBuilder::new(user_id, password_hash)
    }

    pub fn user_id(&self) -> Snowflake {
        self.user_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn password_hash(&self) -> &str {
        self.password_hash.as_ref()
    }
}

impl From<basic_auth::Data> for BasicAuth {
    fn from(value: basic_auth::Data) -> Self {
        BasicAuth {
            user_id: value.user_id.try_into().unwrap(),
            password_hash: value.password_hash,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

pub struct BasicAuthBuilder {
    user_id: Snowflake,

    password_hash: String,
}

impl BasicAuthBuilder {
    pub fn new(user_id: Snowflake, password_hash: String) -> Self {
        BasicAuthBuilder {
            user_id,
            password_hash,
        }
    }

    pub async fn build(self, client: &PrismaClient) -> Result<BasicAuth, QueryError> {
        let data = client
            .basic_auth()
            .create(
                prisma::user::id::equals(self.user_id.to_id_signed()),
                self.password_hash.clone(),
                vec![],
            )
            .exec()
            .await?;

        Ok(BasicAuth {
            user_id: data.user_id.try_into().unwrap(),
            password_hash: data.password_hash,
            created_at: data.created_at.into(),
            updated_at: data.updated_at.into(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Session {
    id: Snowflake,

    user_id: Snowflake,

    blacklisted: bool,
    blacklisted_at: Option<DateTime<Utc>>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UserMetadata {
    id: Snowflake,

    user_id: Snowflake,

    key: String,
    value: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct UserBuilder<'a> {
    id_generator: &'a SnowflakeGenerator,
    client: &'a PrismaClient,

    first_name: Option<String>,
    last_name: Option<String>,
    email_builder: EmailAddressBuilder<'a>,
    basic_auth_builder: Option<BasicAuthBuilder>,

    id: Snowflake,
    application_id: Snowflake,

    user_metadata: Vec<UserMetadata>,
}

impl<'a> UserBuilder<'a> {
    pub fn first_name(&mut self, first_name: String) -> &mut Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(&mut self, last_name: String) -> &mut Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn user_metadata(&mut self, key: String, value: String) -> &mut Self {
        self.user_metadata.push(UserMetadata {
            id: self
                .id_generator
                .next_snowflake()
                .expect("failed to generate snowflake"),
            user_id: self.id,
            key,
            value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });
        self
    }

    pub fn basic_auth(&mut self, password_hash: String) -> &mut Self {
        let builder = BasicAuthBuilder::new(self.id, password_hash);

        self.basic_auth_builder = Some(builder);
        self
    }

    pub async fn build(self) -> Result<User, ModelError> {
        let user_id = self.id;
        let application_id = self.application_id;

        let mut user_create_params: Vec<prisma::user::SetParam> = Vec::new();
        if let Some(first_name) = &self.first_name {
            user_create_params.push(prisma::user::first_name::set(Some(first_name.clone())));
        }

        if let Some(last_name) = &self.last_name {
            user_create_params.push(prisma::user::last_name::set(Some(last_name.clone())));
        }

        if self.basic_auth_builder.is_some() {
            user_create_params.push(prisma::user::password_enabled::set(true));
        }

        let (email_address, basic_auth, user_metadata, user): (
            EmailAddress,
            ModelValue<BasicAuth>,
            Vec<UserMetadata>,
            prisma::user::Data,
        ) = self
            .client
            ._transaction()
            .run::<QueryError, _, _, _>(|client| async move {
                let user_data = client
                    .user()
                    .create(
                        user_id.to_id_signed(),
                        super::prisma::replicated_application::application_id::equals(
                            self.application_id.to_id_signed(),
                        ),
                        user_create_params,
                    )
                    .exec()
                    .await?;

                // Insert and build email address
                let email_address = self
                    .email_builder
                    .build(&client, user_id, self.application_id)
                    .await?;

                // Insert and build basic auth
                let basic_auth = if let Some(basic_auth_builder) = self.basic_auth_builder {
                    ModelValue::Loaded(basic_auth_builder.build(&client).await?)
                } else {
                    ModelValue::NotSet
                };

                let user_metadata = self.user_metadata;
                for metadata in &user_metadata {
                    client
                        .user_metadata()
                        .create(
                            metadata.id.to_id_signed(),
                            prisma::user::id::equals(user_id.to_id_signed()),
                            metadata.key.clone(),
                            metadata.value.clone(),
                            vec![],
                        )
                        .exec()
                        .await?;
                }

                Ok((email_address, basic_auth, user_metadata, user_data))
            })
            .await?;

        let user = User {
            id: user_id,
            application_id,
            first_name: self.first_name,
            last_name: self.last_name,
            email_address: ModelValue::Loaded(email_address),
            basic_auth,
            user_metadata,
            external_users: vec![],
            password_enabled: user.password_enabled,
            last_login_at: None,
            last_login_ip: None,
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
            sessions: vec![],
        };

        Ok(user)
    }
}
