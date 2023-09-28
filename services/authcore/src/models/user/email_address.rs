use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};
use prisma_client_rust::QueryError;

use crate::models::{
    error::ModelError,
    prisma::{self, email_address::Data},
    PrismaClient,
};

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
    pub fn builder(id_generator: &SnowflakeGenerator, user_id: Snowflake) -> EmailAddressBuilder {
        EmailAddressBuilder {
            email_address: EmailAddress {
                id: id_generator.next_snowflake().unwrap(),
                user_id,
                ..Default::default()
            },
        }
    }

    pub async fn find_by_address<C>(
        client: &PrismaClient,
        email: C,
        application_id: Snowflake,
    ) -> Result<EmailAddress, ModelError>
    where
        C: Into<String>,
    {
        let email_address = client
            .email_address()
            .find_first(vec![
                prisma::email_address::email_address::equals(email.into()),
                prisma::email_address::replicated_application_id::equals(
                    application_id.to_id_signed(),
                ),
            ])
            .exec()
            .await?;

        match email_address {
            Some(email_address) => Ok(email_address.into()),
            None => Err(ModelError::NotFound),
        }
    }

    pub async fn get(
        client: &PrismaClient,
        user_id: Snowflake,
        email_id: Snowflake,
        application_id: Snowflake,
    ) -> Result<EmailAddress, ModelError> {
        let email_address = client
            .email_address()
            .find_first(vec![
                prisma::email_address::id::equals(email_id.to_id_signed()),
                prisma::email_address::user_id::equals(user_id.to_id_signed()),
                prisma::email_address::replicated_application_id::equals(
                    application_id.to_id_signed(),
                ),
            ])
            .exec()
            .await?;

        match email_address {
            Some(email_address) => Ok(email_address.into()),
            None => Err(ModelError::NotFound),
        }
    }

    pub async fn set_verified(
        &self,
        client: &PrismaClient,
        application_id: Snowflake,
    ) -> Result<(), ModelError> {
        client
            .email_address()
            .update_many(
                vec![
                    prisma::email_address::id::equals(self.id().to_id_signed()),
                    prisma::email_address::user_id::equals(self.user_id().to_id_signed()),
                    prisma::email_address::replicated_application_id::equals(
                        // TODO: Make application id a part of the email address
                        application_id.to_id_signed(),
                    ),
                ],
                vec![
                    prisma::email_address::verified_at::set(Some(Utc::now().into())),
                    prisma::email_address::verified::set(true),
                    // TODO: Implement verified IP
                ],
            )
            .exec()
            .await?;

        Ok(())
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

impl From<Data> for EmailAddress {
    fn from(value: Data) -> Self {
        Self {
            id: value.id.try_into().unwrap(),
            user_id: value.user_id.try_into().unwrap(),
            email_address: value.email_address,
            verified: value.verified,
            verified_at: value.verified_at.map(|v| v.into()),
            verified_ip: value.verified_ip,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

pub struct EmailAddressBuilder {
    email_address: EmailAddress,
}

impl EmailAddressBuilder {
    pub async fn build(
        self,
        client: &PrismaClient,
        user_id: Snowflake,
        application_id: Snowflake,
    ) -> Result<EmailAddress, QueryError> {
        let email_address = self.email_address;
        let user_id = user_id;

        client
            .email_address()
            .create(
                email_address.id().to_id_signed(),
                prisma::user::id::equals(user_id.to_id_signed()),
                super::prisma::replicated_application::application_id::equals(
                    application_id.to_id_signed(),
                ),
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
