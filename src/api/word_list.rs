// REDO THIS ENTIRE FUNCTION
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::api::AppState;

#[derive(Deserialize)]
pub struct LanguageOptions {
    language: String,
    category: String,
}

#[derive(Serialize)]
struct WordList {
    english: String,
    foreign: String,
    foreign_romanized: String,
}

pub async fn word_list(
    State(pool): State<Arc<AppState>>,
    Json(payload): Json<LanguageOptions>,
) -> Result<String, (StatusCode, String)> {

    let query_result = match payload.language.as_str() {
        "japanese" => {
            sqlx::query_as!(
                WordList,   
                "SELECT english, japanese AS foreign, japanese_romanized AS foreign_romanized FROM vocablist WHERE category = $1",
                payload.category
            )
            .fetch_all(&pool.connection_pool)
            .await
        }
        "korean" => {
            sqlx::query_as!(
                WordList,
                "SELECT english, korean AS foreign, korean_romanized AS foreign_romanized FROM vocablist WHERE category = $1",
                payload.category
            )
            .fetch_all(&pool.connection_pool)
            .await
        }
        "mandarin" => {
            sqlx::query_as!(
                WordList,
                "SELECT english, mandarin AS foreign, mandarin_romanized AS foreign_romanized FROM vocablist WHERE category = $1",
                payload.category
            )
            .fetch_all(&pool.connection_pool)
            .await
        }
        _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "helo".to_string())),
    };

    let json_result = match query_result {
        Ok(result) => serde_json::to_string_pretty(&result),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    return match json_result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}
