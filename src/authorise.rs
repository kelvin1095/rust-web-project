use argon2::{password_hash::PasswordVerifier, Algorithm, Argon2, Params, PasswordHash, Version};
use axum::{http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    iat: i64,
    exp: i64,
}

static HASHED_PASSWORD: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$saltsaltsalt$XELLdwQ1fGhDKj86iwccIiDphgiFYbvmQy04PkSDqgA";

fn check_user(user: User) -> Result<(), (StatusCode, String)> {
    // This should be some check of if:
    // 1. id and/or password is missing send error
    // 2. id and password doesn't match from database send error

    // Pretend this part checks for the sent id in some database and returns the hashed password and pepper.
    if user.id != "admin" {
        return Err((StatusCode::UNAUTHORIZED, "No such user".to_string()));
    }

    let check_password = PasswordHash::new(&HASHED_PASSWORD).unwrap();

    let config = Argon2::new_with_secret(
        b"pepperpepper",
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .unwrap();

    let _ = match config.verify_password(user.password.as_bytes(), &check_password) {
        Ok(_) => (),
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    return Ok(());
}

pub async fn authorize(Json(payload): Json<User>) -> Result<String, (StatusCode, String)> {
    // This function deals with verifying user
    // The client sends some id and password here
    // This is then used to create the Claims struct which then creates the token to send back

    let _ = check_user(payload)?;

    let now = Utc::now();
    let iat = now.timestamp();

    let exp = (now + Duration::weeks(1)).timestamp();

    let test_user = Claims {
        sub: "123456789".to_string(),
        name: "admin".to_string(),
        iat: iat,
        exp: exp,
    };

    let token = encode(
        &Header::default(),
        &test_user,
        &EncodingKey::from_secret(b"secret"),
    )
    .unwrap();

    let new_token: AuthBody = AuthBody {
        token_type: "Bearer".to_string(),
        access_token: token,
    };

    return match serde_json::to_string_pretty(&new_token) {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}
