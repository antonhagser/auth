//! # TOTP 2FA flow
//! TOTP 2FA flow is a flow that allows users to enable and sign in using TOTP 2FA on their accounts.
//!
//! ## Login
//! 1. User logs in with email and password
//! 2. Server checks if the user has TOTP enabled
//! 3. If the user has TOTP enabled, the server returns an error with the code `NeedFurtherVerificationThrough2FA` which contains the user's ID and a TOTP flow token
//! 4. If the user does not have TOTP enabled, the server generates a refresh token and an access token and returns them to the user
//!
//! ## Verify
//! 1. User sends a request to the server with the TOTP flow token and the TOTP code
//! 2. Server checks if the TOTP flow token is valid
//! 3. Server checks if the TOTP code is valid
//! 4. Server generates a refresh token and an access token and returns them to the user
//!
//! ## TOTP 2FA flow token
//! The TOTP 2FA flow token is a signed PASETO that contains the user's ID and the time that the token was created. The token is signed with the server's private key.
//!
//! ## TOTP 2FA flow token verification
//! 1. Server checks if the token is a valid PASETO token
//! 2. Server checks if the token is signed with the server's private key
//! 3. Server checks if the token is expired (created more than 5 minutes ago)
//! 4. Server checks if the token contains the user's ID

use crypto::{snowflake::Snowflake, tokens::jsonwebtoken::Claims};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    models::{prisma::UserTokenType, user::UserToken, PrismaClient},
    state::AppState,
};

/// Represents the default payload of a JWT, containing the subject (sub), issuer (iss), and expiration (exp).
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTokenClaims {
    sub: String,
    iss: String,
    exp: usize,
    token_type: String,
    aud: String,
    jti: Snowflake,
    device_id: Option<String>,
    session_id: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl FlowTokenClaims {
    pub fn token_type(&self) -> &str {
        self.token_type.as_ref()
    }

    pub fn aud(&self) -> &str {
        self.aud.as_ref()
    }

    pub fn device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    pub fn session_id(&self) -> Option<&String> {
        self.session_id.as_ref()
    }

    pub fn ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    pub fn user_agent(&self) -> Option<&String> {
        self.user_agent.as_ref()
    }
}

impl Claims for FlowTokenClaims {
    fn sub(&self) -> &str {
        &self.sub
    }

    fn iss(&self) -> &str {
        &self.iss
    }

    fn exp(&self) -> usize {
        self.exp
    }
}

#[derive(Debug, Error)]
pub enum GenerateFlowTokenError {
    #[error("failed to generate flow token")]
    GenerateToken(#[from] crypto::tokens::jsonwebtoken::Error),
    #[error("failed to store flow token")]
    StoreToken(#[from] crate::models::error::ModelError),
}

/// Generate a TOTP flow token (jwt), store it in the database (currently postgres, should be in-memory later), and return it
pub async fn new_totp_flow_token(
    state: &AppState,
    user_id: Snowflake,
    device_id: Option<String>,
    session_id: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<String, GenerateFlowTokenError> {
    let token_id = state.id_generator().next_snowflake().unwrap();
    let exp = chrono::Utc::now() + chrono::Duration::minutes(5);
    let claims = FlowTokenClaims {
        sub: user_id.to_string(),
        iss: "authcore".to_string(),
        exp: exp.timestamp() as usize,
        token_type: "totp_flow".to_string(),
        aud: "authcore".to_string(),
        jti: token_id,
        device_id,
        session_id,
        ip_address: ip_address.clone(),
        user_agent: user_agent.clone(),
    };

    // Generate a token
    let token = crypto::tokens::jsonwebtoken::JWT::generate_token(claims, state.jwt_priv_key())?;

    // Store the token in the database
    let token = UserToken::builder(token_id, user_id, UserTokenType::TotpFlow, token, exp)
        .ip_address(ip_address)
        .user_agent(user_agent)
        .build(state.prisma())
        .await?;

    Ok(token.token().to_string())
}

#[derive(Debug, Error)]
pub enum VerifyFlowTokenError {
    #[error("failed to verify flow token")]
    VerifyToken(#[from] crypto::tokens::jsonwebtoken::Error),
    #[error("flow token is expired")]
    Expired,
    #[error("flow token is invalid")]
    Invalid,
    #[error("failed to fetch flow token from database")]
    FetchToken(#[from] crate::models::error::ModelError),
}

pub async fn verify_totp_flow_token(
    state: &AppState,
    token: String,
    device_id: Option<String>,
    session_id: Option<String>,
    user_agent: Option<String>,
) -> Result<FlowTokenClaims, VerifyFlowTokenError> {
    // Verify the token
    let claims: FlowTokenClaims =
        crypto::tokens::jsonwebtoken::JWT::verify_token(&token, state.jwt_pub_key())?;

    if let Err(e) = internal_verify_totp_flow_token(
        state.prisma(),
        &token,
        &claims,
        device_id,
        session_id,
        user_agent,
    )
    .await
    {
        // TODO: Delete the token from the database
        // Though remember that multiple tries are allowed, so we should only delete the token if the user has tried too many times
        // Could be done by storing the number of tries in the token itself?

        Err(e)
    } else {
        // TODO: Delete the token from the database

        Ok(claims)
    }
}

async fn internal_verify_totp_flow_token(
    prisma_client: &PrismaClient,
    token: &String,
    claims: &FlowTokenClaims,
    device_id: Option<String>,
    session_id: Option<String>,
    user_agent: Option<String>,
) -> Result<(), VerifyFlowTokenError> {
    // Check if the token is expired
    if chrono::Utc::now().timestamp() as usize > claims.exp() {
        return Err(VerifyFlowTokenError::Expired);
    }

    // Check if the token is valid
    if claims.token_type != "totp_flow" {
        return Err(VerifyFlowTokenError::Invalid);
    }

    // Check if the token contains the user's ID
    let user_id = claims
        .sub
        .parse::<Snowflake>()
        .map_err(|_| VerifyFlowTokenError::Invalid)?;

    // Check if the token contains the device ID
    if let Some(device_id) = device_id {
        if claims.device_id != Some(device_id) {
            return Err(VerifyFlowTokenError::Invalid);
        }
    }

    // Check if the token contains the session ID
    if let Some(session_id) = session_id {
        if claims.session_id != Some(session_id) {
            return Err(VerifyFlowTokenError::Invalid);
        }
    }

    // Check if the token contains the user agent
    if let Some(user_agent) = user_agent {
        if claims.user_agent != Some(user_agent) {
            return Err(VerifyFlowTokenError::Invalid);
        }
    }

    // Fetch token from database and check if it exists
    let database_token =
        UserToken::get(prisma_client, user_id, claims.jti, UserTokenType::TotpFlow).await?;

    // Check if the token matches the one in the database
    if database_token.token() != token {
        return Err(VerifyFlowTokenError::Invalid);
    }

    Ok(())
}
