use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::AppState;

#[derive(Serialize)]
struct Pokemon {
    pokedex_number: Option<i32>,
    name: Option<String>,
    form: Option<String>,
    category: Option<String>,
    type_1: Option<String>,
    type_2: Option<String>,
    ability_1: Option<String>,
    ability_2: Option<String>,
    hidden_ability: Option<String>,
    hp: Option<i32>,
    att: Option<i32>,
    def: Option<i32>,
    spa: Option<i32>,
    spd: Option<i32>,
    spe: Option<i32>,
    height: Option<f32>,
    weight: Option<f32>,
    pokemon_image: Option<String>,
}

#[derive(Deserialize)]
pub struct PokemonTypeQuery {
    type1: String,
    type2: String,
}

#[derive(Serialize, Debug)]
struct PokemonTypeList {
    pokedex_number: Option<i32>,
    name: Option<String>,
    form: Option<String>,
}

pub async fn get_pokemon_data(
    Path(id): Path<String>,
    State(pool): State<Arc<AppState>>,
) -> Result<String, (StatusCode, String)> {
    let pokedex_number = match id.parse::<i32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let query_result = sqlx::query_file_as!(Pokemon, "src/sql/pokemon.sql", pokedex_number)
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

pub async fn get_pokemon_data_by_type(
    State(pool): State<Arc<AppState>>,
    Json(payload): Json<PokemonTypeQuery>,
) -> Result<String, (StatusCode, String)> {
    let query_result = sqlx::query_file_as!(
        PokemonTypeList,
        "src/sql/pokemonByType.sql",
        payload.type1,
        payload.type2
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
