use chrono::{DateTime, Utc};
use crypto::snowflake::Snowflake;

use crate::models::{
    error::ModelError,
    prisma::{self},
    PrismaClient,
};

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct TOTP {
    id: Snowflake,

    user_id: Snowflake,

    secret: String,
    interval: u32,

    totp_backup_codes: Vec<TOTPBackupCode>,

    created_at: DateTime<Utc>,
}

impl TOTP {
    pub fn builder(
        id: Snowflake,
        user_id: Snowflake,
        secret: String,
        interval: u32,
    ) -> TOTPBuilder {
        TOTPBuilder {
            id,
            user_id,
            secret,
            interval,
        }
    }

    pub async fn get(client: &PrismaClient, user_id: Snowflake) -> Result<Self, ModelError> {
        let result = client
            .totp()
            .find_first(vec![prisma::totp::user_id::equals(user_id.to_id_signed())])
            .with(prisma::totp::totp_backup_code::fetch(vec![
                prisma::totp_backup_code::expired::equals(false),
            ]))
            .exec()
            .await?;

        match result {
            Some(result) => Ok(result.into()),
            None => Err(ModelError::NotFound),
        }
    }

    pub async fn verify(&self, client: &PrismaClient, code: String) -> Result<bool, ModelError> {
        // Check against backup codes if the code contains a dash
        if code.contains('-') {
            let backup_code = self
                .totp_backup_codes
                .iter()
                .find(|backup_code| backup_code.code() == code);

            if let Some(backup_code) = backup_code {
                if backup_code.expired() {
                    return Ok(false);
                }

                // Expire the backup code
                self.expire_a_backup_code(client, backup_code.id()).await?;

                return Ok(true);
            } else {
                return Ok(false);
            }
        }

        let res = crypto::totp::verify_totp(&code, self.secret.as_bytes(), self.interval, Some(1));
        Ok(res.unwrap_or(false))
    }

    pub async fn expire_a_backup_code(
        &self,
        client: &PrismaClient,
        code_id: Snowflake,
    ) -> Result<(), ModelError> {
        client
            .totp_backup_code()
            .update(
                prisma::totp_backup_code::id::equals(code_id.to_id_signed()),
                vec![prisma::totp_backup_code::expired::set(true)],
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn secret(&self) -> &str {
        self.secret.as_ref()
    }

    pub fn interval(&self) -> u32 {
        self.interval
    }
}

impl From<prisma::totp::Data> for TOTP {
    fn from(value: prisma::totp::Data) -> Self {
        let totp_backup_codes = value
            .totp_backup_code
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|code| code.into())
            .collect();

        Self {
            id: value.id.try_into().unwrap(),
            user_id: value.user_id.try_into().unwrap(),

            secret: value.secret,
            interval: value.interval as u32,

            totp_backup_codes,

            created_at: value.created_at.into(),
        }
    }
}

pub struct TOTPBuilder {
    id: Snowflake,

    user_id: Snowflake,

    secret: String,
    interval: u32,
}

impl TOTPBuilder {
    pub async fn create(self, client: &PrismaClient) -> Result<TOTP, ModelError> {
        let res = client
            .totp()
            .create(
                self.id.to_id_signed(),
                prisma::user::id::equals(self.user_id.to_id_signed()),
                self.secret,
                vec![prisma::totp::interval::set(self.interval as i32)],
            )
            .exec()
            .await?;

        Ok(res.into())
    }
}

#[derive(Debug, Clone)]
pub struct TOTPBackupCode {
    id: Snowflake,

    code: String,
    expired: bool,

    totp_id: Snowflake,

    created_at: DateTime<Utc>,
}

impl TOTPBackupCode {
    pub fn builder(totp_id: Snowflake, codes: Vec<(Snowflake, String)>) -> TOTPBackupCodeBuilder {
        TOTPBackupCodeBuilder { totp_id, codes }
    }

    pub fn id(&self) -> Snowflake {
        self.id
    }

    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    pub fn expired(&self) -> bool {
        self.expired
    }

    pub fn totp_id(&self) -> Snowflake {
        self.totp_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

impl From<prisma::totp_backup_code::Data> for TOTPBackupCode {
    fn from(value: prisma::totp_backup_code::Data) -> Self {
        Self {
            id: value.id.try_into().unwrap(),

            code: value.code,
            expired: value.expired,

            totp_id: value.totpid.try_into().unwrap(),

            created_at: value.created_at.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TOTPBackupCodeBuilder {
    totp_id: Snowflake,
    codes: Vec<(Snowflake, String)>,
}

impl TOTPBackupCodeBuilder {
    pub async fn create(self, client: &PrismaClient) -> Result<i64, ModelError> {
        let codes: Vec<(i64, String, i64, Vec<prisma::totp_backup_code::SetParam>)> = self
            .codes
            .into_iter()
            .map(|(id, code)| (id.to_id_signed(), code, self.totp_id.to_id_signed(), vec![]))
            .collect();

        let count = client.totp_backup_code().create_many(codes).exec().await?;
        Ok(count)
    }
}
