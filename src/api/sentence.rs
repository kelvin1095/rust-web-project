use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use std::sync::Arc;

use crate::api::AppState;

#[derive(Serialize)]
struct SentenceList {
    english_text: String,
    translated_text: String,
    broken_down: serde_json::Value,
}

pub async fn sentence_list(
    State(pool): State<Arc<AppState>>,
    Path(language): Path<String>,
) -> Result<String, (StatusCode, String)> {
    let query_result = sqlx::query_as!(
        SentenceList,
        "SELECT english_text, translated_text, broken_down 
        FROM sentence_data 
        WHERE language = $1
        ORDER BY RANDOM()
        LIMIT 5;",
        language
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
