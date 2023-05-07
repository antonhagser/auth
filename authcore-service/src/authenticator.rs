use rsa::{pkcs8::LineEnding, RsaPrivateKey, RsaPublicKey};

// Todo: Remove derive debug
#[derive(Debug)]
pub struct Authenticator {
    priv_key_pem: Vec<u8>,
    pub_key_pem: Vec<u8>,
}

impl Authenticator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let priv_key_pem = rsa::pkcs1::EncodeRsaPrivateKey::to_pkcs1_pem(&priv_key, LineEnding::LF)
            .expect("failed to serialize private key to PEM")
            .as_bytes()
            .to_vec();

        let pub_key_pem = rsa::pkcs1::EncodeRsaPublicKey::to_pkcs1_pem(&pub_key, LineEnding::LF)
            .expect("failed to serialize public key to PEM")
            .as_bytes()
            .to_vec();

        Self {
            priv_key_pem,
            pub_key_pem,
        }
    }

    pub fn priv_key_pem(&self) -> &[u8] {
        self.priv_key_pem.as_ref()
    }

    pub fn pub_key_pem(&self) -> &[u8] {
        self.pub_key_pem.as_ref()
    }
}

impl Default for Authenticator {
    fn default() -> Self {
        Self::new()
    }
}
