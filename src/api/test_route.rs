use axum::{extract::State, http::StatusCode, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
// use std::time::Duration;
// use tokio::time::sleep;

use crate::api::api_response::ApiResponse;
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
}

type T = Total;

pub async fn list_things(
    State(pool): State<Arc<AppState>>,
    cookie: TypedHeader<Cookie>,
    Json(payload): Json<Summation>,
) -> Result<String, (StatusCode, String)> {
    // let help = sleep(Duration::from_secs(5)).await;

    let auth_token = cookie.get("auth-token");

    match auth_token {
        Some(token) => is_authorised(&pool.jwt_secret, token)?,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                ApiResponse::<T>::error("Please Log in".to_string()),
            ))
        }
    };

    let num1_parsed = match payload.num1.parse::<f32>() {
        Ok(result) => result,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiResponse::<T>::error(err.to_string()),
            ))
        }
    };

    let num2_parsed = match payload.num2.parse::<f32>() {
        Ok(result) => result,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiResponse::<T>::error(err.to_string()),
            ))
        }
    };

    let total: Total = Total {
        total: num1_parsed + num2_parsed,
    };

    return Ok(ApiResponse::success(total));
}
