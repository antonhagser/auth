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
    user_id: Snowflake,
    token_type: UserTokenType,
    token: String,
    expires_at: DateTime<Utc>,
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
                vec![],
            )
            .exec()
            .await?;

        Ok(data.into())
    }
}
