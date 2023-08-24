use chrono::{DateTime, Utc};
use crypto::snowflake::Snowflake;
use prisma_client_rust::QueryError;

use crate::models::{
    prisma::{self, basic_auth},
    PrismaClient,
};

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
