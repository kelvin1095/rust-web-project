// I think it is better to redo the database storing words and translation
// so that this can be written easier.
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::api::AppState;

#[derive(Deserialize)]
pub struct LanguageOptions {
    category: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct WordList {
    english: String,
    translated: String,
    romanized: String,
}

pub async fn word_list(
    State(pool): State<Arc<AppState>>,
    Path(language): Path<String>,
    Json(payload): Json<LanguageOptions>,
) -> Result<String, (StatusCode, String)> {
    let query_result = sqlx::query_as!(
        WordList,
        "SELECT english, translated, romanized FROM word_data
        WHERE language = $1 
        AND category = $2
        ORDER BY RANDOM()
        LIMIT 5;",
        language,
        payload.category
    )
    .fetch_all(&pool.connection_pool)
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
