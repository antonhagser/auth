use std::vec;

use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};

use crate::models::{
    error::ModelError,
    prisma::{user_token::Data, UserTokenType},
    PrismaClient,
};

pub struct UserToken {
    id: Snowflake,
    user_id: Snowflake,

    token_type: UserTokenType,
    token: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl UserToken {
    pub fn builder(id_generator: &SnowflakeGenerator) -> UserTokenBuilder {
        UserTokenBuilder {
            id_generator,
            user_id: None,
            token_type: None,
            token: None,
            expires_at: None,
        }
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
}

impl From<Data> for UserToken {
    fn from(data: Data) -> Self {
        Self {
            id: data.id.try_into().unwrap(),
            user_id: data.user_id.try_into().unwrap(),

            token_type: data.token_type,
            token: data.token,

            created_at: data.created_at.into(),
            updated_at: data.updated_at.into(),
            expires_at: data.expires_at.into(),
        }
    }
}

pub struct UserTokenBuilder<'a> {
    id_generator: &'a SnowflakeGenerator,
    user_id: Option<Snowflake>,
    token_type: Option<UserTokenType>,
    token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl<'a> UserTokenBuilder<'a> {
    pub fn user_id(mut self, user_id: Snowflake) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn token_type(mut self, token_type: UserTokenType) -> Self {
        self.token_type = Some(token_type);
        self
    }

    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn expires_at(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    pub async fn build(self, client: &PrismaClient) -> Result<UserToken, ModelError> {
        // Verify that the fields have been assigned
        let user_id = self
            .user_id
            .ok_or(ModelError::MissingField("user_id".to_owned()))?;
        let token_type = self
            .token_type
            .ok_or(ModelError::MissingField("token_type".to_owned()))?;
        let token = self
            .token
            .ok_or(ModelError::MissingField("token".to_owned()))?;
        let expires_at = self
            .expires_at
            .ok_or(ModelError::MissingField("expires_at".to_owned()))?;

        let id = self.id_generator.next_snowflake().unwrap();
        let data = client
            .user_token()
            .create(
                id.to_id_signed(),
                super::prisma::user::id::equals(user_id.to_id_signed()),
                token_type,
                token,
                expires_at.into(),
                vec![],
            )
            .exec()
            .await?;

        Ok(data.into())
    }
}
