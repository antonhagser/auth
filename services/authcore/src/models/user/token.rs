use std::vec;

use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};

use crate::models::{
    error::ModelError,
    prisma::{user_token::Data, UserTokenType},
    PrismaClient,
};

#[derive(Debug, Clone)]
pub struct UserToken {
    id: Snowflake,
    user_id: Snowflake,

    token_type: UserTokenType,
    token: String,

    ip_address: Option<String>,
    user_agent: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl UserToken {
    pub fn builder(
        id_generator: &SnowflakeGenerator,
        user_id: Snowflake,
        token_type: UserTokenType,
        token: String,
        expires_at: DateTime<Utc>,
    ) -> UserTokenBuilder {
        UserTokenBuilder {
            id_generator,
            user_id,
            token_type,
            token,
            expires_at,

            ip_address: None,
            user_agent: None,
        }
    }

    /// Get a user token by user_id, token and token_type.
    pub async fn get(
        client: &PrismaClient,
        user_id: Snowflake,
        token: String,
        token_type: UserTokenType,
    ) -> Result<Self, ModelError> {
        let data = client
            .user_token()
            .find_first(vec![
                super::prisma::user_token::token::equals(token),
                super::prisma::user_token::token_type::equals(token_type),
                super::prisma::user_token::user_id::equals(user_id.to_id_signed()),
            ])
            .exec()
            .await?;

        if data.is_none() {
            return Err(ModelError::NotFound);
        }

        Ok(data.unwrap().into())
    }

    /// User token ID.
    pub fn id(&self) -> Snowflake {
        self.id
    }

    /// User ID.
    pub fn user_id(&self) -> Snowflake {
        self.user_id
    }

    /// User token type.
    pub fn token_type(&self) -> UserTokenType {
        self.token_type
    }

    /// User token.
    pub fn token(&self) -> &str {
        self.token.as_ref()
    }

    /// User token creation time.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// User token update time.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// User token expiration time.
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    pub fn user_agent(&self) -> Option<&String> {
        self.user_agent.as_ref()
    }
}

impl From<Data> for UserToken {
    fn from(data: Data) -> Self {
        Self {
            id: data.id.try_into().unwrap(),
            user_id: data.user_id.try_into().unwrap(),

            token_type: data.token_type,
            token: data.token,

            ip_address: data.ip_address,
            user_agent: data.user_agent,

            created_at: data.created_at.into(),
            updated_at: data.updated_at.into(),
            expires_at: data.expires_at.into(),
        }
    }
}

pub struct UserTokenBuilder<'a> {
    id_generator: &'a SnowflakeGenerator,
    user_id: Snowflake,
    token_type: UserTokenType,
    token: String,
    expires_at: DateTime<Utc>,

    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl<'a> UserTokenBuilder<'a> {
    pub async fn build(self, client: &PrismaClient) -> Result<UserToken, ModelError> {
        let id = self.id_generator.next_snowflake().unwrap();
        let data = client
            .user_token()
            .create(
                id.to_id_signed(),
                super::prisma::user::id::equals(self.user_id.to_id_signed()),
                self.token_type,
                self.token,
                self.expires_at.into(),
                vec![
                    super::prisma::user_token::ip_address::set(self.ip_address),
                    super::prisma::user_token::user_agent::set(self.user_agent),
                ],
            )
            .exec()
            .await?;

        Ok(data.into())
    }

    pub fn ip_address(mut self, ip_address: Option<String>) -> Self {
        self.ip_address = ip_address;
        self
    }

    pub fn user_agent(mut self, user_agent: Option<String>) -> Self {
        self.user_agent = user_agent;
        self
    }
}
