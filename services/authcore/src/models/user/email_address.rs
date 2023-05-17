use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};
use prisma_client_rust::QueryError;

use crate::models::{error::ModelError, prisma, PrismaClient};

#[derive(Debug, Clone, Default)]
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
    pub fn builder(
        id_generator: &SnowflakeGenerator,
        user_id: Snowflake,
    ) -> EmailAddressBuilder<'_> {
        EmailAddressBuilder {
            id_generator,
            email_address: EmailAddress {
                id: id_generator.next_snowflake().unwrap(),
                user_id,
                ..Default::default()
            },
        }
    }

    pub async fn exists<C>(prisma: &PrismaClient, email: C) -> Result<bool, ModelError>
    where
        C: Into<String>,
    {
        let email_address = prisma
            .email_address()
            .find_first(vec![prisma::email_address::email_address::equals(
                email.into(),
            )])
            .exec()
            .await?;

        Ok(email_address.is_some())
    }

    pub fn id(&self) -> Snowflake {
        self.id
    }

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

    pub fn email_address(&self) -> &str {
        self.email_address.as_ref()
    }
}

pub struct EmailAddressBuilder<'a> {
    id_generator: &'a SnowflakeGenerator,
    email_address: EmailAddress,
}

impl<'a> EmailAddressBuilder<'a> {
    pub async fn build(
        self,
        prisma: &PrismaClient,
        user_id: Snowflake,
    ) -> Result<EmailAddress, QueryError> {
        let email_address = self.email_address;
        let user_id = user_id;

        prisma
            .email_address()
            .create(
                self.id_generator.next_snowflake().unwrap().to_id_signed(),
                prisma::user::id::equals(user_id.to_id_signed()),
                email_address.email_address().to_owned(),
                vec![],
            )
            .exec()
            .await?;

        Ok(email_address)
    }

    pub fn email_address<C>(mut self, email_address: C) -> Self
    where
        C: Into<String>,
    {
        self.email_address.email_address = email_address.into();
        self
    }
}
