use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use crypto::snowflake::Snowflake;
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    core::totp,
    http::{modules::get_request, response::HTTPResponse},
    models::user::{
        totp::{TOTPBackupCode, TOTP},
        User, UserWith,
    },
    state::AppState,
};

#[derive(Deserialize)]
pub struct SetupRequest {
    token: String,
}

#[derive(Serialize)]
pub struct SetupResponse {}

pub async fn route(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> (StatusCode, Json<HTTPResponse>) {
    let (parts, body) = request.into_parts();

    // Accept multiple different ways to give the data, url encoded form data or json body
    let data: SetupRequest = match get_request(&parts, body).await {
        Some(d) => d,
        None => {
            let response = HTTPResponse::error(
                "BadRequest",
                "Invalid content type, expected application/x-www-form-urlencoded or application/json".to_owned(),
                (),
            );

            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    let user_id = Snowflake::try_from(34347795687145472u64).unwrap();
    let user = match User::get(state.prisma(), user_id, vec![UserWith::TOTP]).await {
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

    // Verify the totp flow token
    let res = totp::verify_totp_flow_token(&state, data.token, None, None, None, None).await;
    if res.is_err() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(HTTPResponse::error(
                "Unauthorized",
                "Invalid token".to_owned(),
                (),
            )),
        );
    }

    // Generate a new totp secret
    let totp_secret = crypto::tokens::string::random_token();
    let base32_secret = crypto::totp::BASE32_NOPAD.encode(totp_secret.as_bytes());
    let totp_interval = 30;

    // Try to generate, verify that it succeeds
    if crypto::totp::generate_totp(base32_secret.as_bytes(), totp_interval).is_err() {
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
    // TODO: Interesting fucking issue, can't use rand thread_rng because ???
    let backup_codes: [[u8; 6]; 10] = rand::random();

    // Combine backup codes into 6 digit codes
    let backup_codes: Vec<String> = backup_codes
        .iter()
        .map(|code| {
            code.iter()
                .map(|digit| digit.to_string())
                .collect::<String>()
        })
        .collect();

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

    let response = HTTPResponse::ok(SetupResponse {});
    (StatusCode::OK, Json(response))
}
