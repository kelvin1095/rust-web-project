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
struct HelpMe {
    english: String,
    translated: String,
    romanized: String,
}

pub async fn word_list(
    State(pool): State<Arc<AppState>>,
    Path(language): Path<String>,
    Json(payload): Json<LanguageOptions>,
) -> Result<String, (StatusCode, String)> {
    println!("{language}");
    println!("{:?}", payload.category);
    let query_result = sqlx::query_as!(
        HelpMe, 
        "SELECT english, translated, romanized FROM word_data WHERE language = $1 AND category = $2",
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
