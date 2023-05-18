use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};
use prisma_client_rust::QueryError;

use self::email_address::EmailAddressBuilder;

use super::{
    error::ModelError,
    prisma::{self, user::Data, PrismaClient},
    ModelValue,
};

pub use email_address::EmailAddress;
pub use external_user::ExternalUser;

mod email_address;
mod external_user;

#[derive(Debug, Clone)]
pub struct User {
    id: Snowflake,

    // Todo: Replace Option with a new type which implements a enum None (exists but not loaded), NotSet (not set), Some (set to a value)
    first_name: Option<String>,
    last_name: Option<String>,

    email_address: ModelValue<EmailAddress>,

    external_users: Vec<ExternalUser>,

    password_enabled: bool,
    basic_auth: Option<BasicAuth>,

    last_login_at: Option<DateTime<Utc>>,
    last_login_ip: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    sessions: Vec<Session>,
    user_tokens: Vec<UserToken>,
    user_metadata: Vec<UserMetadata>,

    application_id: Snowflake,
}

impl User {
    pub async fn exists<C>(prisma: &PrismaClient, email: C) -> Result<bool, ModelError>
    where
        C: Into<String>,
    {
        EmailAddress::exists(prisma, email).await
    }

    pub fn builder<'a>(
        id_generator: &'a SnowflakeGenerator,
        prisma: &'a PrismaClient,
        application_id: Snowflake,
        email: String,
    ) -> UserBuilder<'a> {
        let user_id = id_generator.next_snowflake().unwrap();
        let email_builder = EmailAddress::builder(id_generator, user_id).email_address(email);

        UserBuilder {
            id_generator,
            prisma,

            first_name: None,
            last_name: None,
            email_builder,
            basic_auth: None,

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

    pub fn basic_auth(&self) -> Option<&BasicAuth> {
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

    pub fn user_tokens(&self) -> &[UserToken] {
        self.user_tokens.as_ref()
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
        User {
            id: value.id.try_into().unwrap(),
            first_name: value.first_name,
            last_name: value.last_name,
            email_address: ModelValue::NotLoaded,
            external_users: vec![],
            password_enabled: value.password_enabled,
            basic_auth: None,
            last_login_at: value.last_login_at.map(|v| v.into()),
            last_login_ip: value.last_login_ip,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            sessions: vec![],
            user_tokens: vec![],
            user_metadata: vec![],
            application_id: value.application_id.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicAuth {
    id: Snowflake,

    user_id: Snowflake,

    username: String,
    password_hash: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BasicAuth {
    pub fn user_id(&self) -> Snowflake {
        self.user_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
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
pub struct UserToken {
    id: Snowflake,

    user_id: Snowflake,

    token_type: super::prisma::UserTokenType,
    token: String,

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
    prisma: &'a PrismaClient,

    first_name: Option<String>,
    last_name: Option<String>,
    email_builder: EmailAddressBuilder<'a>,
    basic_auth: Option<BasicAuth>,

    id: Snowflake,
    application_id: Snowflake,

    user_metadata: Vec<UserMetadata>,
}

impl<'a> UserBuilder<'a> {
    pub fn first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn user_metadata(mut self, key: String, value: String) -> Self {
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

    pub fn basic_auth(mut self, username: String, password_hash: String) -> Self {
        self.basic_auth = Some(BasicAuth {
            id: self
                .id_generator
                .next_snowflake()
                .expect("failed to generate snowflake"),
            user_id: self.id,
            username,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        self
    }

    pub async fn build(self) -> Result<User, ModelError> {
        // Todo: Make sure that application exists?

        let user_id = self.id;
        let application_id = self.application_id;

        let mut user_create_params: Vec<prisma::user::SetParam> = Vec::new();
        if let Some(first_name) = &self.first_name {
            user_create_params.push(prisma::user::first_name::set(Some(first_name.clone())));
        }

        if let Some(last_name) = &self.last_name {
            user_create_params.push(prisma::user::last_name::set(Some(last_name.clone())));
        }

        if self.basic_auth.is_some() {
            user_create_params.push(prisma::user::password_enabled::set(true));
        }

        let result: (
            EmailAddress,
            Option<BasicAuth>,
            Vec<UserMetadata>,
            prisma::user::Data,
        ) = self
            .prisma
            ._transaction()
            .run::<QueryError, _, _, _>(|client| async move {
                let user_data = client
                    .user()
                    .create(
                        user_id.to_id_signed(),
                        application_id.to_id_signed(),
                        user_create_params,
                    )
                    .exec()
                    .await?;

                // Insert and build email address
                let email_address = self.email_builder.build(&client, user_id).await?;

                let basic_auth = if let Some(basic_auth) = self.basic_auth {
                    client
                        .basic_auth()
                        .create(
                            basic_auth.id.to_id_signed(),
                            prisma::user::id::equals(user_id.to_id_signed()),
                            basic_auth.username.clone(),
                            basic_auth.password_hash.clone(),
                            vec![],
                        )
                        .exec()
                        .await?;

                    Some(basic_auth)
                } else {
                    None
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
            email_address: ModelValue::Some(result.0),
            basic_auth: result.1,
            user_metadata: result.2,

            external_users: vec![],
            password_enabled: result.3.password_enabled,
            last_login_at: None,
            last_login_ip: None,
            created_at: result.3.created_at.into(),
            updated_at: result.3.updated_at.into(),
            sessions: vec![],
            user_tokens: vec![],
        };

        Ok(user)
    }
}
