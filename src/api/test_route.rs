use axum::{extract::State, http::StatusCode, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
// use std::time::Duration;
// use tokio::time::sleep;

use crate::api::check_jwt::is_authorised;
use crate::api::AppState;

#[derive(Deserialize)]
pub struct Summation {
    num1: String,
    num2: String,
}

#[derive(Serialize)]
struct Total {
    total: f32,
    method: String,
}

pub async fn list_things(
    State(pool): State<Arc<AppState>>,
    cookie: TypedHeader<Cookie>,
    Json(payload): Json<Summation>,
) -> Result<String, (StatusCode, String)> {
    // let help = sleep(Duration::from_secs(5)).await;

    let auth_token = cookie.get("auth-token").unwrap();
    let _ = is_authorised(&pool.jwt_secret, auth_token)?;

    let num1_parsed = match payload.num1.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let num2_parsed = match payload.num2.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let total: Total = Total {
        total: num1_parsed + num2_parsed,
        method: "Post".to_string(),
    };

    let result = serde_json::to_string_pretty(&total);

    return match result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}
