use chrono::{DateTime, Utc};
use crypto::snowflake::Snowflake;
use prisma_client_rust::QueryError;

use super::{error::ModelError, ModelValue, PrismaClient};

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

    pub async fn find_by_id(prisma: &PrismaClient, id: Snowflake) -> Result<Self, ModelError> {
        let app = prisma
            .replicated_application()
            .find_first(vec![
                super::prisma::replicated_application::application_id::equals(id.to_id_signed()),
            ])
            .exec()
            .await?;

        match app {
            Some(app) => Ok(ReplicatedApplication::from(app)),
            None => Err(ModelError::RecordNotFound),
        }
    }

    pub async fn find_by_id_with_config(
        prisma: &PrismaClient,
        id: Snowflake,
    ) -> Result<Self, ModelError> {
        let app = prisma
            .replicated_application()
            .find_first(vec![
                super::prisma::replicated_application::application_id::equals(id.to_id_signed()),
            ])
            .with(super::prisma::replicated_application::basic_auth_config::fetch())
            .exec()
            .await?;

        match app {
            Some(app) => Ok(ReplicatedApplication::from(app)),
            None => Err(ModelError::RecordNotFound),
        }
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

impl From<super::prisma::replicated_application::Data> for ReplicatedApplication {
    fn from(value: super::prisma::replicated_application::Data) -> Self {
        // Check if email address is loaded
        let basic_auth_config = if let Some(basic_auth_config) = value.basic_auth_config {
            if let Some(basic_auth_config) = basic_auth_config {
                ModelValue::Loaded(BasicAuthConfig::from(*basic_auth_config))
            } else {
                ModelValue::NotSet
            }
        } else {
            ModelValue::NotLoaded
        };

        Self {
            application_id: value.application_id.try_into().unwrap(),
            basic_auth_enabled: value.basic_auth_enabled,
            basic_auth_config,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

pub struct BasicAuthConfig {
    application_id: Snowflake,

    enable_password_strength_check: bool,
    zxcvbn_minimum_score: u8,

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

    pub fn enable_password_strength_check(&self) -> bool {
        self.enable_password_strength_check
    }

    pub fn zxcvbn_minimum_score(&self) -> u8 {
        self.zxcvbn_minimum_score
    }
}

impl From<super::prisma::basic_auth_config::Data> for BasicAuthConfig {
    fn from(value: super::prisma::basic_auth_config::Data) -> Self {
        Self {
            application_id: value.application_id.try_into().unwrap(),

            enable_password_strength_check: value.enable_password_strength_check,
            zxcvbn_minimum_score: value.zxcvbn_min_score.try_into().unwrap(),

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

pub struct BasicAuthConfigBuilderStrict {
    min_uppercase: u8,
    min_lowercase: u8,
    min_numbers: u8,
    min_symbols: u8,
}

pub struct BasicAuthConfigBuilderZxcvbn {
    zxcvbn_minimum_score: u8,
}

pub struct BasicAuthConfigBuilder {
    min_password_length: Option<u8>,
    max_password_length: Option<u8>,

    /// Defaults to true with minimum score of 2
    password_strength_check: Option<BasicAuthConfigBuilderZxcvbn>,

    enable_strict_password: Option<BasicAuthConfigBuilderStrict>,
}

impl BasicAuthConfigBuilder {
    pub fn new() -> Self {
        Self {
            min_password_length: None,
            max_password_length: None,

            password_strength_check: None,

            enable_strict_password: None,
        }
    }

    pub fn min_password_length(&mut self, min_password_length: u8) -> &mut Self {
        self.min_password_length = Some(min_password_length);
        self
    }

    pub fn max_password_length(&mut self, max_password_length: u8) -> &mut Self {
        self.max_password_length = Some(max_password_length);
        self
    }

    pub fn change_password_strength_check(&mut self, zxcvbn_minimum_score: u8) -> &mut Self {
        self.password_strength_check = Some(BasicAuthConfigBuilderZxcvbn {
            zxcvbn_minimum_score,
        });
        self
    }

    pub fn enable_strict_password(
        &mut self,
        min_uppercase: u8,
        min_lowercase: u8,
        min_numbers: u8,
        min_symbols: u8,
    ) -> &mut Self {
        self.enable_strict_password = Some(BasicAuthConfigBuilderStrict {
            min_uppercase,
            min_lowercase,
            min_numbers,
            min_symbols,
        });
        self
    }

    pub async fn build(
        self,
        client: &PrismaClient,
        application_id: Snowflake,
    ) -> Result<BasicAuthConfig, QueryError> {
        let mut create_params = Vec::new();

        if let Some(min_password_length) = self.min_password_length {
            create_params.push(super::prisma::basic_auth_config::min_password_length::set(
                min_password_length.into(),
            ));
        }

        if let Some(max_password_length) = self.max_password_length {
            create_params.push(super::prisma::basic_auth_config::max_password_length::set(
                max_password_length.into(),
            ));
        }

        if let Some(enable_strict_password) = self.enable_strict_password {
            create_params.push(super::prisma::basic_auth_config::enable_strict_password::set(true));
            create_params.push(super::prisma::basic_auth_config::min_uppercase::set(
                enable_strict_password.min_uppercase.into(),
            ));
            create_params.push(super::prisma::basic_auth_config::min_lowercase::set(
                enable_strict_password.min_lowercase.into(),
            ));
            create_params.push(super::prisma::basic_auth_config::min_numbers::set(
                enable_strict_password.min_numbers.into(),
            ));
            create_params.push(super::prisma::basic_auth_config::min_symbols::set(
                enable_strict_password.min_symbols.into(),
            ));
        }

        if let Some(password_strength_check) = self.password_strength_check {
            create_params
                .push(super::prisma::basic_auth_config::enable_password_strength_check::set(true));
            create_params.push(super::prisma::basic_auth_config::zxcvbn_min_score::set(
                password_strength_check.zxcvbn_minimum_score.into(),
            ));
        }

        let data = client
            .basic_auth_config()
            .create(
                super::prisma::replicated_application::application_id::equals(
                    application_id.to_id_signed(),
                ),
                create_params,
            )
            .exec()
            .await?;

        Ok(BasicAuthConfig {
            application_id: data.application_id.try_into().unwrap(),

            enable_password_strength_check: data.enable_password_strength_check,
            zxcvbn_minimum_score: data.zxcvbn_min_score.try_into().unwrap(),

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
