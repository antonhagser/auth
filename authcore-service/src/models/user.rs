use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};
use prisma_client_rust::QueryError;

use super::{
    error::ModelError,
    prisma::{self, PrismaClient},
};

#[derive(Debug, Clone)]
pub struct User {
    id: Snowflake,

    first_name: Option<String>,
    last_name: Option<String>,

    email_address: EmailAddress,

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
    pub fn builder<'a>(
        id_generator: &'a SnowflakeGenerator,
        prisma: &'a PrismaClient,
        application_id: Snowflake,
    ) -> UserBuilder<'a> {
        let user = User {
            id: id_generator
                .next_snowflake()
                .expect("failed to generate snowflake"),
            first_name: None,
            last_name: None,
            email_address: EmailAddress {
                id: id_generator
                    .next_snowflake()
                    .expect("failed to generate snowflake"),
                user_id: id_generator
                    .next_snowflake()
                    .expect("failed to generate snowflake"),
                email_address: String::new(),
                verified: false,
                verified_at: None,
                verified_ip: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            external_users: vec![],
            password_enabled: false,
            basic_auth: None,
            last_login_at: None,
            last_login_ip: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            sessions: vec![],
            user_tokens: vec![],
            user_metadata: vec![],
            application_id,
        };

        UserBuilder {
            id_generator,
            prisma,
            user,
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

    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
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
}

pub struct UserBuilder<'a> {
    id_generator: &'a SnowflakeGenerator,
    prisma: &'a PrismaClient,
    user: User,
}

impl<'a> UserBuilder<'a> {
    pub fn first_name(mut self, first_name: String) -> Self {
        self.user.first_name = Some(first_name);
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.user.last_name = Some(last_name);
        self
    }

    pub fn email_address<C: Into<String>>(mut self, email_address: C) -> Self {
        self.user.email_address.email_address = email_address.into();
        self
    }

    pub fn user_metadata(mut self, key: String, value: String) -> Self {
        self.user.user_metadata.push(UserMetadata {
            id: self
                .id_generator
                .next_snowflake()
                .expect("failed to generate snowflake"),
            user_id: self.user.id,
            key,
            value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });
        self
    }

    pub fn basic_auth(mut self, username: String, password_hash: String) -> Self {
        self.user.basic_auth = Some(BasicAuth {
            id: self
                .id_generator
                .next_snowflake()
                .expect("failed to generate snowflake"),
            user_id: self.user.id,
            username,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        self.user.password_enabled = true;
        self
    }

    pub async fn build(self) -> Result<User, ModelError> {
        let user = self.user;
        let user_id = user.id;
        let application_id = user.application_id;

        let mut user_create_params: Vec<prisma::user::SetParam> = Vec::new();
        if let Some(first_name) = &user.first_name {
            user_create_params.push(prisma::user::first_name::set(Some(first_name.clone())));
        }

        if let Some(last_name) = &user.last_name {
            user_create_params.push(prisma::user::last_name::set(Some(last_name.clone())));
        }

        if user.basic_auth.is_some() {
            user_create_params.push(prisma::user::password_enabled::set(true));
        }

        let result: (prisma::user::Data, prisma::email_address::Data, User) = self
            .prisma
            ._transaction()
            .run::<QueryError, _, _, _>(|client| async move {
                let user_data = client
                    .user()
                    .create(
                        user_id.to_id_signed(),
                        prisma::application::id::equals(application_id.to_id_signed()),
                        vec![],
                    )
                    .exec()
                    .await?;

                let email_address_data = client
                    .email_address()
                    .create(
                        user.email_address.id.to_id_signed(),
                        prisma::user::id::equals(user_id.to_id_signed()),
                        user.email_address.email_address.clone(),
                        vec![],
                    )
                    .exec()
                    .await?;

                if let Some(basic_auth) = &user.basic_auth {
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
                }

                for metadata in &user.user_metadata {
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

                Ok((user_data, email_address_data, user))
            })
            .await?;

        // Todo: Update create and update times

        Ok(result.2)
    }
}

#[derive(Debug, Clone)]
pub struct EmailAddress {
    id: Snowflake,

    user_id: Snowflake,

    email_address: String,

    verified: bool,
    verified_at: Option<DateTime<Utc>>,
    verified_ip: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl EmailAddress {
    pub fn user_id(&self) -> Snowflake {
        self.user_id
    }

    pub fn verified(&self) -> bool {
        self.verified
    }

    pub fn verified_at(&self) -> Option<DateTime<Utc>> {
        self.verified_at
    }

    pub fn verified_ip(&self) -> Option<&String> {
        self.verified_ip.as_ref()
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
pub struct ExternalUser {
    id: Snowflake,

    user_id: Snowflake,

    provider: String,
    provider_user_id: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
