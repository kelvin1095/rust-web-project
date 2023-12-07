use argon2::{password_hash::Salt, Algorithm, Argon2, Params, PasswordHasher, Version};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::api::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

pub async fn new_user(
    State(pool): State<Arc<AppState>>,
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

    let insert_new_user = sqlx::query_file!(
        "src/sql/createUser.sql",
        payload.username,
        payload.email,
        phc,
        pepper_string
    )
    .execute(&pool.connection_pool)
    .await;

    let _ = match insert_new_user {
        Ok(_) => (),
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    return Ok(());
}
