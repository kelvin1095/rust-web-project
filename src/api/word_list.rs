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

#[derive(Default, Serialize, sqlx::FromRow)]
#[sqlx(default)]
struct HelpMe {
    english: String,
    japanese: Option<String>,
    japanese_romanized: Option<String>,
    korean: Option<String>,
    korean_romanized: Option<String>,
    mandarin: Option<String>,
    mandarin_romanized: Option<String>,
}

pub async fn word_list(
    State(pool): State<Arc<AppState>>,
    Path(language): Path<String>,
    Json(payload): Json<LanguageOptions>,
) -> Result<String, (StatusCode, String)> {
    let category = payload.category;
    let query = format!(
        "SELECT english, {language}, {language}_romanized FROM vocablist WHERE category = '{category}';" // "SELECT english, {language}, {language}_romanized FROM vocablist WHERE category = 'Family';"
    );

    let query_result = sqlx::query_as::<_, HelpMe>(&query)
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
