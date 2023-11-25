use argon2::{password_hash::PasswordVerifier, Algorithm, Argon2, Params, PasswordHash, Version};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
struct AuthBody {
    token_type: String,
    access_token: String,
}

#[derive(Deserialize)]
pub struct User {
    id: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserDetails {
    user_id: i32,
    pepper_value: String,
    hashed_password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    iat: i64,
    exp: i64,
}

static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET_KEY").expect("missing secret key");
    return secret;
});

pub async fn authorize(
    State(pool): State<PgPool>,
    Json(payload): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_match = sqlx::query_file_as!(UserDetails, "src/sql/matchUser.sql", payload.id)
        .fetch_one(&pool)
        .await;

    let result: UserDetails = match user_match {
        Ok(result) => result,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "User ID not found".to_string())),
    };

    let check_password = PasswordHash::new(&result.hashed_password).unwrap();

    let config = Argon2::new_with_secret(
        result.pepper_value.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .unwrap();

    let _ = match config.verify_password(payload.password.as_bytes(), &check_password) {
        Ok(_) => (),
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::weeks(1)).timestamp();

    let test_user = Claims {
        sub: result.user_id.to_string(),
        name: payload.id,
        iat: iat,
        exp: exp,
    };

    let token = encode(
        &Header::default(),
        &test_user,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .unwrap();

    let token_header = "auth-token=".to_owned() + &token + "; Path=/; Secure; SameSite=Strict";

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, token_header.as_str().parse().unwrap());

    return Ok(headers);
}
