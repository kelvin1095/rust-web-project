use axum::{extract::State, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;

use crate::api::AppState;

#[derive(Serialize)]
struct Sentence {
    english_text: String,
    japanese_text: String,
    japanese_broken_down: serde_json::Value,
}

pub async fn sentence(State(pool): State<Arc<AppState>>) -> Result<String, (StatusCode, String)> {
    let query_result = sqlx::query_as!(
        Sentence,
        "SELECT english_text, japanese_text, japanese_broken_down FROM language_data WHERE id = 1;",
    )
    .fetch_one(&pool.connection_pool)
    .await;

    let json_result = match query_result {
        Ok(result) => serde_json::to_string_pretty(&result),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    return match json_result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}
