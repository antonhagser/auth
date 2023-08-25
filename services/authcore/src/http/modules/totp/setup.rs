use std::{net::SocketAddr, str::FromStr};

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use crypto::{snowflake::Snowflake, totp};
use hyper::{Body, Request, StatusCode};
use serde::Serialize;
use tracing::info;

use crate::{
    core::token,
    http::response::HTTPResponse,
    models::{
        prisma,
        user::{
            totp::{TOTPBackupCode, TOTP},
            User, UserWith,
        },
    },
    state::AppState,
};

#[derive(Serialize)]
pub struct SetupResponse {
    totp_secret: String,
    interval: u32,
    backup_codes: Vec<String>,
}

pub async fn route(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
    // Get authorization header
    // ! Temporary till middleware is implemented
    let auth = match request.headers().get("Authorization") {
        Some(auth) => auth,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(HTTPResponse::error(
                    "Unauthorized",
                    "Missing authorization header".to_owned(),
                    (),
                )),
            );
        }
    }
    .to_str()
    .unwrap();

    // Parse the authorization header
    let auth = match auth.split(' ').collect::<Vec<_>>().as_slice() {
        ["Bearer", token] => token,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(HTTPResponse::error(
                    "Unauthorized",
                    "Invalid authorization header".to_owned(),
                    (),
                )),
            );
        }
    }
    .to_owned();

    // Verify the access token
    let access_token = token::verify_access_token(&state, auth);
    let access_token = match access_token {
        Ok(access_token) => access_token,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(HTTPResponse::error(
                    "Unauthorized",
                    "Invalid access token".to_owned(),
                    (),
                )),
            );
        }
    };

    let user = match User::get(
        state.prisma(),
        Snowflake::from_str(access_token.subject().unwrap()).unwrap(),
        vec![UserWith::TOTP],
    )
    .await
    {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HTTPResponse::error(
                    "InternalServerError",
                    "Failed to get user".to_owned(),
                    (),
                )),
            );
        }
    };

    // Check if user has totp enabled
    if user.totp().is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(HTTPResponse::error(
                "BadRequest",
                "TOTP already enabled".to_owned(),
                (),
            )),
        );
    }

    // Generate a new totp secret
    let totp_secret = crypto::totp::generate_totp_secret();
    let totp_interval = 30;

    // Try to generate, verify that it succeeds
    let totp = crypto::totp::generate_totp(totp_secret.as_bytes(), totp_interval);
    if totp.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(HTTPResponse::error(
                "InternalServerError",
                "Failed to generate totp and could therefore not setup totp".to_owned(),
                (),
            )),
        );
    }

    let transaction = state.prisma()._transaction().begin().await;
    let (transaction_controller, prisma_client) = match transaction {
        Ok(transaction) => transaction,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HTTPResponse::error(
                    "InternalServerError",
                    "Failed to setup totp".to_owned(),
                    (),
                )),
            );
        }
    };

    // Create the totp
    let totp = TOTP::builder(
        state.id_generator().next_snowflake().unwrap(),
        user.id(),
        totp_secret,
        totp_interval,
    )
    .create(state.prisma())
    .await;

    // Check if totp was created
    let totp = match totp {
        Ok(totp) => totp,
        Err(_) => {
            let _ = transaction_controller.rollback(prisma_client).await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HTTPResponse::error(
                    "InternalServerError",
                    "Failed to setup totp".to_owned(),
                    (),
                )),
            );
        }
    };

    // Generate backup codes
    let backup_codes = (0..10)
        .map(|_| totp::generate_backup_code())
        .collect::<Vec<_>>();

    // Create the backup codes
    let res = TOTPBackupCode::builder(
        totp.id(),
        backup_codes
            .iter()
            .map(|code| {
                (
                    state.id_generator().next_snowflake().unwrap(),
                    code.to_owned(),
                )
            })
            .collect(),
    )
    .create(state.prisma())
    .await;

    // Rollback if totp backup code insert fails
    if res.is_err() {
        let _ = transaction_controller.rollback(prisma_client).await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(HTTPResponse::error(
                "InternalServerError",
                "Failed to setup totp".to_owned(),
                (),
            )),
        );
    }

    // Update user to check totp enabled flag
    let res = state
        .prisma()
        .user()
        .update(
            prisma::user::id::equals(user.id().to_id_signed()),
            vec![prisma::user::totp_enabled::set(true)],
        )
        .exec()
        .await;

    // Rollback if user update fails
    if res.is_err() {
        let _ = transaction_controller.rollback(prisma_client).await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(HTTPResponse::error(
                "InternalServerError",
                "Failed to setup totp".to_owned(),
                (),
            )),
        );
    }

    // Commit the transaction
    if (transaction_controller.commit(prisma_client).await).is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(HTTPResponse::error(
                "InternalServerError",
                "Failed to create totp".to_owned(),
                (),
            )),
        );
    }

    info!(
        "enabled totp for user {} with interval {}",
        user.id().to_id_signed(),
        totp_interval
    );

    // Return the totp secret and interval
    let response = HTTPResponse::ok(SetupResponse {
        totp_secret: totp.secret().to_string(),
        interval: totp.interval(),
        backup_codes,
    });

    (StatusCode::OK, Json(response))
}
