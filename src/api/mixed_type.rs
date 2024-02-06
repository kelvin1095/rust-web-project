use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;

use crate::api::api_response::ApiResponse;
use crate::api::AppState;

#[derive(Serialize, Clone, Debug)]
struct WordList {
    english: String,
    translated: String,
    romanized: String,
}

#[derive(Serialize)]
struct SentenceList {
    english_text: String,
    translated_text: String,
    broken_down: serde_json::Value,
}

#[derive(Serialize)]
enum QuizType {
    WordList(Vec<WordList>),
    SentenceList(SentenceList),
}

type T = Vec<QuizType>;

pub async fn quiz_list(
    State(pool): State<Arc<AppState>>,
    Path(language): Path<String>,
) -> Result<String, (StatusCode, String)> {
    let sentence_result = sqlx::query_as!(
        SentenceList,
        "SELECT english_text, translated_text, broken_down 
        FROM sentence_data
        WHERE language = $1 
        ORDER BY RANDOM()
        LIMIT 5;",
        language,
    )
    .fetch_all(&pool.connection_pool)
    .await
    .unwrap();

    let word_result = sqlx::query_as!(
        WordList,
        "SELECT english, translated, romanized 
        FROM word_data
        WHERE language = $1 
        ORDER BY RANDOM()
        LIMIT 25;",
        language,
    )
    .fetch_all(&pool.connection_pool)
    .await
    .unwrap();

    let grouped_word_list: Vec<Vec<WordList>> =
        word_result.chunks(5).map(|chunk| chunk.to_vec()).collect();

    // println!("{:?}", grouped_word_list);

    let mut sentence_result_quiz: Vec<QuizType> = Vec::new();
    for word in grouped_word_list {
        sentence_result_quiz.push(QuizType::WordList(word))
    }
    for sentence in sentence_result {
        sentence_result_quiz.push(QuizType::SentenceList(sentence))
    }
    let mut rng = thread_rng();
    sentence_result_quiz.shuffle(&mut rng);

    // let json_result = serde_json::to_string_pretty(&sentence_result_quiz);

    return Ok(ApiResponse::<T>::success(sentence_result_quiz));
}
