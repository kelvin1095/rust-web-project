use std::sync::Arc;

use argon2::{password_hash::PasswordVerifier, Algorithm, Argon2, Params, PasswordHash, Version};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::api::api_response::ApiResponse;
use crate::api::jwt_claims::Claims;
use crate::api::AppState;

#[derive(Serialize)]
struct UserDetails {
    username: String,
}

#[derive(Deserialize)]
pub struct UserLoginDetails {
    user_id: i32,
    username: String,
    pepper_value: String,
    hashed_password: String,
}

type T = UserDetails;

pub async fn authorize(
    State(pool): State<Arc<AppState>>,
    credential: TypedHeader<Authorization<Basic>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_match = sqlx::query_file_as!(
        UserLoginDetails,
        "src/sql/matchUser.sql",
        credential.username()
    )
    .fetch_one(&pool.connection_pool)
    .await;

    let result = match user_match {
        Ok(result) => result,
        Err(err) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                ApiResponse::<T>::error(err.to_string()),
            ))
        }
    };

    let check_password = PasswordHash::new(&result.hashed_password).unwrap();

    let config = Argon2::new_with_secret(
        result.pepper_value.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .unwrap();

    let _ = match config.verify_password(credential.password().as_bytes(), &check_password) {
        Ok(_) => (),
        Err(err) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                ApiResponse::<T>::error(err.to_string()),
            ))
        }
    };

    let now = Utc::now();

    let user_claim = Claims {
        sub: result.user_id.to_string(),
        name: credential.username().to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::weeks(1)).timestamp(),
    };

    let token = encode(
        &Header::default(),
        &user_claim,
        &EncodingKey::from_secret(pool.jwt_secret.as_bytes()),
    )
    .unwrap();

    let token_header = "auth-token=".to_owned()
        + &token
        + "; Max-Age=86400; Path=/; HttpOnly; Secure; SameSite=Strict";

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, token_header.as_str().parse().unwrap());

    let username = UserDetails {
        username: result.username,
    };

    return Ok((headers, ApiResponse::<T>::success(username)));
}
