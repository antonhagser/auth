use chrono::{DateTime, Utc};
use crypto::{input::password::PasswordRequirements, snowflake::Snowflake};
use prisma_client_rust::QueryError;

use super::{error::ModelError, PrismaClient};

pub use super::prisma::EmailVerificationType;

#[derive(Debug, Clone)]
pub struct ReplicatedApplication {
    application_id: Snowflake,

    basic_auth_enabled: bool,

    basic_auth_config: Option<BasicAuthConfig>,
    verification_config: Option<VerificationConfig>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ReplicatedApplication {
    pub async fn new_and_insert(
        client: &PrismaClient,
        application_id: Snowflake,
        domain_name: String,
        basic_auth_config_builder: BasicAuthConfigBuilder,
        verification_config_builder: VerificationConfigBuilder,
    ) -> Result<Self, QueryError> {
        let (app, basic_auth_cfg, verification_cfg): (
            super::prisma::replicated_application::Data,
            BasicAuthConfig,
            VerificationConfig,
        ) = client
            ._transaction()
            .run::<QueryError, _, _, _>(|client| async move {
                // Insert replicated application
                let d1_app = client
                    .replicated_application()
                    .create(application_id.to_id_signed(), domain_name, vec![])
                    .exec()
                    .await?;

                // Insert basic auth config
                let basic_auth_config = basic_auth_config_builder
                    .build(&client, application_id)
                    .await?;

                // Insert verification config
                let verification_config = verification_config_builder
                    .build(&client, application_id)
                    .await?;

                Ok((d1_app, basic_auth_config, verification_config))
            })
            .await?;

        let app = Self {
            application_id,
            basic_auth_enabled: app.basic_auth_enabled,
            basic_auth_config: Some(basic_auth_cfg),
            verification_config: Some(verification_cfg),
            created_at: app.created_at.into(),
            updated_at: app.updated_at.into(),
        };

        Ok(app)
    }

    pub async fn delete(
        client: &PrismaClient,
        application_id: Snowflake,
    ) -> Result<(), QueryError> {
        client
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

    pub async fn get(client: &PrismaClient, id: Snowflake) -> Result<Self, ModelError> {
        let app = client
            .replicated_application()
            .find_first(vec![
                super::prisma::replicated_application::application_id::equals(id.to_id_signed()),
            ])
            .exec()
            .await?;

        match app {
            Some(app) => Ok(ReplicatedApplication::from(app)),
            None => Err(ModelError::NotFound),
        }
    }

    pub async fn find_by_id_with_config(
        client: &PrismaClient,
        id: Snowflake,
    ) -> Result<Self, ModelError> {
        let app = client
            .replicated_application()
            .find_first(vec![
                super::prisma::replicated_application::application_id::equals(id.to_id_signed()),
            ])
            .with(super::prisma::replicated_application::basic_auth_config::fetch())
            .with(super::prisma::replicated_application::verification_config::fetch())
            .exec()
            .await?;

        match app {
            Some(app) => Ok(ReplicatedApplication::from(app)),
            None => Err(ModelError::NotFound),
        }
    }

    pub fn application_id(&self) -> Snowflake {
        self.application_id
    }

    pub fn basic_auth_enabled(&self) -> bool {
        self.basic_auth_enabled
    }

    pub async fn basic_auth_config(&mut self, client: &PrismaClient) -> BasicAuthConfig {
        // If config is present, unwrap it and return
        if let Some(cfg) = &self.basic_auth_config {
            cfg.clone()
        } else {
            // Otherwise fetch config from database
            let cfg = client
                .basic_auth_config()
                .find_first(vec![
                    super::prisma::basic_auth_config::application_id::equals(
                        self.application_id.to_id_signed(),
                    ),
                ])
                .exec()
                .await
                .unwrap();

            // Todo: verify that unwrap is the expected behavior
            let cfg = BasicAuthConfig::from(cfg.unwrap());

            // Update config
            self.basic_auth_config = Some(cfg.clone());

            cfg
        }
    }

    pub async fn verification_config(&mut self, client: &PrismaClient) -> VerificationConfig {
        // If config is present, unwrap it and return
        if let Some(cfg) = &self.verification_config {
            cfg.clone()
        } else {
            // Otherwise fetch config from database
            let cfg = client
                .verification_config()
                .find_first(vec![
                    super::prisma::verification_config::application_id::equals(
                        self.application_id.to_id_signed(),
                    ),
                ])
                .exec()
                .await
                .unwrap();

            // TODO: verify that unwrap is the expected behavior
            let cfg = VerificationConfig::from(cfg.unwrap());

            // Update config
            self.verification_config = Some(cfg.clone());

            cfg
        }
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
        let basic_auth_config = match value.basic_auth_config {
            Some(basic_auth_config) => {
                basic_auth_config.map(|basic_auth_config| BasicAuthConfig::from(*basic_auth_config))
            }
            None => None,
        };

        let verification_config = match value.verification_config {
            Some(verification_config) => verification_config
                .map(|verification_config| VerificationConfig::from(*verification_config)),
            None => None,
        };

        Self {
            application_id: value.application_id.try_into().unwrap(),
            basic_auth_enabled: value.basic_auth_enabled,
            basic_auth_config,
            verification_config,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn as_password_requirements_config(&self) -> PasswordRequirements {
        PasswordRequirements {
            min_length: self.min_password_length,
            max_length: self.max_password_length,

            enable_strict_requirements: self.enable_strict_password,
            min_uppercase: self.min_uppercase,
            min_lowercase: self.min_lowercase,
            min_numbers: self.min_numbers,
            min_symbols: self.min_symbols,

            min_zxcvbn_score: self.zxcvbn_minimum_score,
        }
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

#[derive(Debug, Clone)]
pub struct VerificationConfig {
    application_id: Snowflake,

    email_redirect_url: Option<String>,
    expires_after: u32,
    email_verification_type: EmailVerificationType,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl VerificationConfig {
    pub fn builder() -> VerificationConfigBuilder {
        VerificationConfigBuilder::new()
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

    pub fn email_redirect_url(&self) -> Option<&String> {
        self.email_redirect_url.as_ref()
    }

    pub fn expires_after(&self) -> u32 {
        self.expires_after
    }

    pub fn email_verification_type(&self) -> &EmailVerificationType {
        &self.email_verification_type
    }
}

impl From<super::prisma::verification_config::Data> for VerificationConfig {
    fn from(value: super::prisma::verification_config::Data) -> Self {
        Self {
            application_id: value.application_id.try_into().unwrap(),

            email_redirect_url: value.email_redirect_url,
            expires_after: value.expires_after.try_into().unwrap(),
            email_verification_type: value.email_verification_type,

            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

pub struct VerificationConfigBuilder {
    email_redirect_url: Option<String>,
    expires_after: Option<u32>,
    email_verification_type: Option<EmailVerificationType>,
}

impl VerificationConfigBuilder {
    pub fn new() -> Self {
        Self {
            email_redirect_url: None,
            expires_after: None,
            email_verification_type: None,
        }
    }

    pub fn email_redirect_url(&mut self, email_redirect_url: String) -> &mut Self {
        self.email_redirect_url = Some(email_redirect_url);
        self
    }

    pub fn expires_after(&mut self, expires_after: u32) -> &mut Self {
        self.expires_after = Some(expires_after);
        self
    }

    pub fn email_verification_type(
        &mut self,
        email_verification_type: EmailVerificationType,
    ) -> &mut Self {
        self.email_verification_type = Some(email_verification_type);
        self
    }

    pub async fn build(
        self,
        client: &PrismaClient,
        application_id: Snowflake,
    ) -> Result<VerificationConfig, QueryError> {
        let mut create_params = Vec::new();

        if let Some(email_redirect_url) = self.email_redirect_url {
            create_params.push(super::prisma::verification_config::email_redirect_url::set(
                Some(email_redirect_url),
            ));
        }

        if let Some(expires_after) = self.expires_after {
            create_params.push(super::prisma::verification_config::expires_after::set(
                expires_after as i32,
            ));
        }

        if let Some(email_verification_type) = self.email_verification_type {
            create_params.push(
                super::prisma::verification_config::email_verification_type::set(
                    email_verification_type,
                ),
            );
        }

        let data = client
            .verification_config()
            .create(
                super::prisma::replicated_application::application_id::equals(
                    application_id.to_id_signed(),
                ),
                create_params,
            )
            .exec()
            .await?;

        Ok(VerificationConfig {
            application_id: data.application_id.try_into().unwrap(),

            email_redirect_url: data.email_redirect_url,
            expires_after: data.expires_after.try_into().unwrap(),
            email_verification_type: data.email_verification_type,

            created_at: data.created_at.into(),
            updated_at: data.updated_at.into(),
        })
    }
}

impl Default for VerificationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
