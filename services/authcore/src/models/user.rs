use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};
use prisma_client_rust::QueryError;

use self::{
    basic_auth::{BasicAuth, BasicAuthBuilder},
    email_address::EmailAddressBuilder,
    totp::TOTP,
};

use super::{
    error::ModelError,
    prisma::{self, user::Data, PrismaClient},
};

pub use email_address::EmailAddress;
pub use external_user::ExternalUser;
pub use token::UserToken;

pub mod basic_auth;
pub mod email_address;
pub mod external_user;
pub mod token;
pub mod totp;

#[derive(Debug, Clone)]
pub struct User {
    id: Snowflake,

    first_name: Option<String>,
    last_name: Option<String>,

    email_address: Option<EmailAddress>,

    password_enabled: bool,
    basic_auth: Option<BasicAuth>,

    totp_enabled: bool,
    totp: Option<TOTP>,

    last_login_at: Option<DateTime<Utc>>,
    last_login_ip: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    application_id: Snowflake, // replicatedApplicationID
}

pub enum UserWith {
    EmailAddress,
    BasicAuth,
    TOTP,
}

impl User {
    pub async fn get<'a>(
        client: &PrismaClient,
        id: Snowflake,
        with: Vec<UserWith>,
    ) -> Result<User, ModelError> {
        let mut user = client
            .user()
            .find_first(vec![prisma::user::id::equals(id.to_id_signed())]);

        // Add with params (joins)
        for w in with {
            user = match w {
                UserWith::EmailAddress => user.with(prisma::user::email_address::fetch()),
                UserWith::BasicAuth => user.with(prisma::user::basic_auth::fetch()),
                UserWith::TOTP => user.with(prisma::user::totp::fetch()),
            };
        }

        // Execute query
        let user = user.exec().await.map_err(ModelError::DatabaseError)?;

        match user {
            Some(user) => Ok(user.into()),
            None => Err(ModelError::NotFound),
        }
    }

    pub async fn find_by_email<C>(
        client: &PrismaClient,
        email: C,
        application_id: Snowflake,
        user_with: Vec<UserWith>,
    ) -> Result<User, ModelError>
    where
        C: Into<String>,
    {
        // The way this works is by searching in the email_address table for the email address
        // and then joining the user table to get the user data.
        let mut user_fetch = super::prisma::email_address::user::fetch();
        for with in user_with {
            user_fetch = match with {
                UserWith::EmailAddress => user_fetch,
                UserWith::BasicAuth => user_fetch.with(super::prisma::user::basic_auth::fetch()),
                UserWith::TOTP => user_fetch.with(super::prisma::user::totp::fetch()),
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
                let user = *email_address.user.take().ok_or(ModelError::NotFound)?;

                // Convert to user
                let mut user: User = user.into();

                let email_address: EmailAddress = email_address.into();
                user.email_address = Some(email_address);
                Ok(user)
            }
            None => Err(ModelError::NotFound),
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

    pub fn email_address(&self) -> Option<&EmailAddress> {
        self.email_address.as_ref()
    }

    pub fn password_enabled(&self) -> bool {
        self.password_enabled
    }

    pub async fn basic_auth(
        &mut self,
        try_from_server: Option<&PrismaClient>,
    ) -> Option<&BasicAuth> {
        if self.basic_auth.is_some() {
            return self.basic_auth.as_ref();
        }

        if let Some(client) = try_from_server {
            let basic_auth = client
                .basic_auth()
                .find_first(vec![prisma::basic_auth::user_id::equals(
                    self.id.to_id_signed(),
                )])
                .exec()
                .await
                .unwrap()
                .map(|data| data.into());

            self.basic_auth = basic_auth;
        }

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

    pub fn application_id(&self) -> Snowflake {
        self.application_id
    }

    pub fn totp_enabled(&self) -> bool {
        self.totp_enabled
    }

    pub fn totp(&self) -> Option<&TOTP> {
        self.totp.as_ref()
    }
}

impl From<Data> for User {
    fn from(value: Data) -> Self {
        let email_address = match value.email_address {
            Some(Some(email_address)) => Some((*email_address).into()),
            Some(None) => None,
            None => None,
        };

        let basic_auth = match value.basic_auth {
            Some(Some(basic_auth)) => Some((*basic_auth).into()),
            Some(None) => None,
            None => None,
        };

        let totp = match value.totp {
            Some(Some(totp)) => Some((*totp).into()),
            Some(None) => None,
            None => None,
        };

        User {
            // Parse fields that must have a value
            id: value.id.try_into().unwrap(),
            first_name: value.first_name,
            last_name: value.last_name,
            password_enabled: value.password_enabled,
            last_login_ip: value.last_login_ip,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            last_login_at: value.last_login_at.map(|v| v.into()),
            application_id: value.replicated_application_id.try_into().unwrap(),
            totp_enabled: value.totp_enabled,

            // Parse fields that may not have a value
            email_address,
            basic_auth,
            totp,
        }
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

        // Insert user with prisma using transaction
        let (email_address, basic_auth, _, user): (
            EmailAddress,
            Option<BasicAuth>,
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
                let basic_auth = match self.basic_auth_builder {
                    Some(basic_auth_builder) => Some(basic_auth_builder.build(&client).await?),
                    None => None,
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
            email_address: Some(email_address),
            basic_auth,
            password_enabled: user.password_enabled,
            last_login_at: None,
            last_login_ip: None,
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
            totp_enabled: false,
            totp: None,
        };

        Ok(user)
    }
}
