use chrono::{DateTime, Utc};
use crypto::snowflake::{Snowflake, SnowflakeGenerator};

use super::{error::ModelError, user::User, PrismaClient};

#[derive(Debug, Clone)]
pub struct Application {
    id: Snowflake,

    name: String,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,

    users: Vec<User>,
}

impl Application {
    pub fn builder<'a>(
        id_generator: &'a SnowflakeGenerator,
        prisma: &'a PrismaClient,
    ) -> ApplicationBuilder<'a> {
        let application = Application {
            id: id_generator
                .next_snowflake()
                .expect("failed to generate snowflake"),
            name: String::from("default"),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            users: vec![],
        };

        ApplicationBuilder {
            prisma,
            application,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn users(&self) -> &[User] {
        self.users.as_ref()
    }

    pub fn id(&self) -> Snowflake {
        self.id
    }
}

pub struct ApplicationBuilder<'a> {
    prisma: &'a PrismaClient,
    application: Application,
}

impl<'a> ApplicationBuilder<'a> {
    pub fn name(mut self, name: String) -> Self {
        self.application.name = name;
        self
    }

    pub async fn build(self) -> Result<Application, ModelError> {
        let application = self.application;
        let application_id = application.id;

        let application = self
            .prisma
            .application()
            .create(application_id.to_id_signed(), application.name, vec![])
            .exec()
            .await?;

        let application = Application {
            id: application.id.into(),
            name: application.name,
            created_at: application.created_at.into(),
            updated_at: application.updated_at.into(),
            users: vec![],
        };

        Ok(application)
    }
}
