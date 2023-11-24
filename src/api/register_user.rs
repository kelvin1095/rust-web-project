use argon2::{password_hash::Salt, Algorithm, Argon2, Params, PasswordHasher, Version};
use axum::{
    debug_handler,
    extract::{Json, State},
    http::StatusCode,
};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[debug_handler]
pub async fn new_user(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<(), (StatusCode, String)> {
    let salt_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let pepper_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let salt = Salt::from_b64(salt_string.as_str()).unwrap();

    let config = Argon2::new_with_secret(
        pepper_string.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .unwrap();

    let phc = config
        .hash_password(payload.password.as_bytes(), salt)
        .unwrap()
        .to_string();

    let _ = sqlx::query_file!(
        "src/sql/createUser.sql",
        payload.username,
        payload.email,
        phc,
        pepper_string
    )
    .execute(&pool)
    .await;

    return Ok(());
}
