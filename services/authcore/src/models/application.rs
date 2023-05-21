use chrono::{DateTime, Utc};
use crypto::snowflake::Snowflake;
use prisma_client_rust::QueryError;

use super::{ModelValue, PrismaClient};

pub struct ReplicatedApplication {
    application_id: Snowflake,

    basic_auth_enabled: bool,
    basic_auth_config: ModelValue<BasicAuthConfig>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ReplicatedApplication {
    pub async fn new_and_insert(
        prisma: &PrismaClient,
        application_id: Snowflake,
        basic_auth_config_builder: BasicAuthConfigBuilder,
    ) -> Result<Self, QueryError> {
        let (app, cfg): (super::prisma::replicated_application::Data, BasicAuthConfig) = prisma
            ._transaction()
            .run::<QueryError, _, _, _>(|client| async move {
                // Insert replicated application
                let d1_app = client
                    .replicated_application()
                    .create(application_id.to_id_signed(), vec![])
                    .exec()
                    .await?;

                // Insert basic auth config
                let basic_auth_config = basic_auth_config_builder
                    .build(&client, application_id)
                    .await?;

                Ok((d1_app, basic_auth_config))
            })
            .await?;

        let app = Self {
            application_id,
            basic_auth_enabled: app.basic_auth_enabled,
            basic_auth_config: ModelValue::Loaded(cfg),
            created_at: app.created_at.into(),
            updated_at: app.updated_at.into(),
        };

        Ok(app)
    }

    pub async fn delete(
        prisma: &PrismaClient,
        application_id: Snowflake,
    ) -> Result<(), QueryError> {
        prisma
            .replicated_application()
            .delete(
                super::prisma::replicated_application::application_id::equals(
                    application_id.to_id_signed(),
                ),
            )
            .exec()
            .await?;

        Ok(())
    }

    pub fn application_id(&self) -> Snowflake {
        self.application_id
    }

    pub fn basic_auth_enabled(&self) -> bool {
        self.basic_auth_enabled
    }

    pub fn basic_auth_config(&self) -> &ModelValue<BasicAuthConfig> {
        &self.basic_auth_config
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub struct BasicAuthConfig {
    application_id: Snowflake,

    password_strength: super::prisma::PasswordStrength,

    min_password_length: u8,
    max_password_length: u8,

    enable_strict_password: bool,
    min_uppercase: u8,
    min_lowercase: u8,
    min_numbers: u8,
    min_symbols: u8,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BasicAuthConfig {
    pub fn builder() -> BasicAuthConfigBuilder {
        BasicAuthConfigBuilder::new()
    }

    pub fn application_id(&self) -> Snowflake {
        self.application_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn password_strength(&self) -> super::prisma::PasswordStrength {
        self.password_strength
    }

    pub fn min_password_length(&self) -> u8 {
        self.min_password_length
    }

    pub fn max_password_length(&self) -> u8 {
        self.max_password_length
    }

    pub fn enable_strict_password(&self) -> bool {
        self.enable_strict_password
    }

    pub fn min_uppercase(&self) -> u8 {
        self.min_uppercase
    }

    pub fn min_lowercase(&self) -> u8 {
        self.min_lowercase
    }

    pub fn min_numbers(&self) -> u8 {
        self.min_numbers
    }

    pub fn min_symbols(&self) -> u8 {
        self.min_symbols
    }
}

impl From<Box<super::prisma::basic_auth_config::Data>> for BasicAuthConfig {
    fn from(value: std::boxed::Box<super::prisma::basic_auth_config::Data>) -> Self {
        Self {
            application_id: value.application_id.try_into().unwrap(),

            password_strength: value.password_strength,

            min_password_length: value.min_password_length.try_into().unwrap(),
            max_password_length: value.max_password_length.try_into().unwrap(),

            enable_strict_password: value.enable_strict_password,

            min_uppercase: value.min_uppercase.try_into().unwrap(),
            min_lowercase: value.min_lowercase.try_into().unwrap(),
            min_numbers: value.min_numbers.try_into().unwrap(),
            min_symbols: value.min_symbols.try_into().unwrap(),

            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

pub struct BasicAuthConfigBuilder {
    password_strength: super::prisma::PasswordStrength,

    min_password_length: u8,
    max_password_length: u8,

    enable_strict_password: bool,
    min_uppercase: u8,
    min_lowercase: u8,
    min_numbers: u8,
    min_symbols: u8,
}

impl BasicAuthConfigBuilder {
    pub fn new() -> Self {
        Self {
            password_strength: super::prisma::PasswordStrength::Average,

            min_password_length: 8,
            max_password_length: 128,

            enable_strict_password: false,
            min_uppercase: 0,
            min_lowercase: 0,
            min_numbers: 0,
            min_symbols: 0,
        }
    }

    pub fn password_strength(
        &mut self,
        password_strength: super::prisma::PasswordStrength,
    ) -> &mut Self {
        self.password_strength = password_strength;
        self
    }

    pub fn min_password_length(&mut self, min_password_length: u8) -> &mut Self {
        self.min_password_length = min_password_length;
        self
    }

    pub fn max_password_length(&mut self, max_password_length: u8) -> &mut Self {
        self.max_password_length = max_password_length;
        self
    }

    pub fn enable_strict_password(&mut self, enable_strict_password: bool) -> &mut Self {
        self.enable_strict_password = enable_strict_password;
        self
    }

    pub fn min_uppercase(&mut self, min_uppercase: u8) -> &mut Self {
        self.min_uppercase = min_uppercase;
        self
    }

    pub fn min_lowercase(&mut self, min_lowercase: u8) -> &mut Self {
        self.min_lowercase = min_lowercase;
        self
    }

    pub fn min_numbers(&mut self, min_numbers: u8) -> &mut Self {
        self.min_numbers = min_numbers;
        self
    }

    pub fn min_symbols(&mut self, min_symbols: u8) -> &mut Self {
        self.min_symbols = min_symbols;
        self
    }

    pub async fn build(
        self,
        client: &PrismaClient,
        application_id: Snowflake,
    ) -> Result<BasicAuthConfig, QueryError> {
        let data = client
            .basic_auth_config()
            .create(
                super::prisma::replicated_application::application_id::equals(
                    application_id.to_id_signed(),
                ),
                vec![
                    super::prisma::basic_auth_config::password_strength::set(
                        self.password_strength,
                    ),
                    super::prisma::basic_auth_config::min_password_length::set(
                        self.min_password_length.try_into().unwrap(),
                    ),
                    super::prisma::basic_auth_config::max_password_length::set(
                        self.max_password_length.try_into().unwrap(),
                    ),
                    super::prisma::basic_auth_config::enable_strict_password::set(
                        self.enable_strict_password,
                    ),
                    super::prisma::basic_auth_config::min_uppercase::set(
                        self.min_uppercase.try_into().unwrap(),
                    ),
                    super::prisma::basic_auth_config::min_lowercase::set(
                        self.min_lowercase.try_into().unwrap(),
                    ),
                    super::prisma::basic_auth_config::min_numbers::set(
                        self.min_numbers.try_into().unwrap(),
                    ),
                    super::prisma::basic_auth_config::min_symbols::set(
                        self.min_symbols.try_into().unwrap(),
                    ),
                ],
            )
            .exec()
            .await?;

        Ok(BasicAuthConfig {
            application_id: data.application_id.try_into().unwrap(),

            password_strength: data.password_strength,

            min_password_length: data.min_password_length.try_into().unwrap(),
            max_password_length: data.max_password_length.try_into().unwrap(),

            enable_strict_password: data.enable_strict_password,

            min_uppercase: data.min_uppercase.try_into().unwrap(),
            min_lowercase: data.min_lowercase.try_into().unwrap(),
            min_numbers: data.min_numbers.try_into().unwrap(),
            min_symbols: data.min_symbols.try_into().unwrap(),

            created_at: data.created_at.into(),
            updated_at: data.updated_at.into(),
        })
    }
}

impl Default for BasicAuthConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
