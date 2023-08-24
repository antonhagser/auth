//! This module defines the server's global state, which is shared across
//! all routes and services.

use std::sync::Arc;

use crypto::{
    snowflake::SnowflakeGenerator,
    tokens::{
        jsonwebtoken::rsa,
        paseto::{self, SymmetricKey},
    },
};
use rand::{rngs::StdRng, SeedableRng};

use crate::{models::PrismaClient, ServiceData};

pub use config::{Config, CONFIG};

mod config;

/// `State` represents the server's global state.
pub struct State {
    prisma_client: PrismaClient,
    id_generator: SnowflakeGenerator,
    service_data: ServiceData,
    paseto_key: SymmetricKey,
    jwt_priv_key: Vec<u8>,
    jwt_pub_key: Vec<u8>,
}

impl State {
    pub fn new(
        prisma_client: PrismaClient,
        id_generator: SnowflakeGenerator,
        service_data: ServiceData,
        paseto_key: &[u8],
        jwt_seed: &[u8],
    ) -> Self {
        let keys = paseto::generate_key(paseto_key);

        // Derive a seed for the RNG from the JWT seed
        let seed: [u8; 32] = {
            let mut tmp = [0u8; 32];
            for (i, &byte) in jwt_seed.iter().enumerate() {
                tmp[i % 32] ^= byte;
            }
            tmp
        };

        // Seed the RNG with the derived seed
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let bits = 2048;

        let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = rsa::RsaPublicKey::from(&priv_key);

        let priv_jwt_key =
            rsa::pkcs1::EncodeRsaPrivateKey::to_pkcs1_pem(&priv_key, rsa::pkcs8::LineEnding::LF)
                .expect("failed to serialize private key to PEM")
                .as_bytes()
                .to_vec();

        let pub_key_pem =
            rsa::pkcs1::EncodeRsaPublicKey::to_pkcs1_pem(&pub_key, rsa::pkcs8::LineEnding::LF)
                .expect("failed to serialize public key to PEM")
                .as_bytes()
                .to_vec();

        Self {
            prisma_client,
            id_generator,
            service_data,
            paseto_key: keys,
            jwt_priv_key: priv_jwt_key,
            jwt_pub_key: pub_key_pem,
        }
    }

    pub fn prisma(&self) -> &PrismaClient {
        &self.prisma_client
    }

    pub fn id_generator(&self) -> &SnowflakeGenerator {
        &self.id_generator
    }

    pub fn config(&self) -> &Config {
        &CONFIG
    }

    pub fn service_data(&self) -> &ServiceData {
        &self.service_data
    }

    pub fn paseto_key(&self) -> &SymmetricKey {
        &self.paseto_key
    }

    pub fn jwt_priv_key(&self) -> &[u8] {
        self.jwt_priv_key.as_ref()
    }

    pub fn jwt_pub_key(&self) -> &[u8] {
        self.jwt_pub_key.as_ref()
    }
}

/// `AppState` is an alias for an `Arc<State>` to provide shared ownership
/// and thread-safe access to the server's global state.
pub type AppState = Arc<State>;
