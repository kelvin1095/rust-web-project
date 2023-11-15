use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    //   routing::get,
    //   Router
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

// use std::time::Duration;
// use sqlx::postgres::PgPoolOptions;

#[derive(Serialize)]
struct Pokemon {
    pokedexnumber: Option<i32>,
    name: Option<String>,
    form: Option<String>,
    type1: Option<String>,
    type2: Option<String>,
    ability1: Option<String>,
    ability2: Option<String>,
    hiddenability: Option<String>,
    hp: Option<i32>,
    att: Option<i32>,
    def: Option<i32>,
    spa: Option<i32>,
    spd: Option<i32>,
    spe: Option<i32>,
    height: Option<f32>,
    weight: Option<f32>,
    pokemonimagefilename: Option<String>,
}

#[derive(Deserialize)]
pub struct PokemonTypeQuery {
    type1: String,
    type2: String,
}

#[derive(Serialize, Debug)]
struct PokemonTypeList {
    pokedexnumber: Option<i32>,
    name: Option<String>,
    form: Option<String>,
    type1: Option<String>,
    type2: Option<String>,
}

pub async fn get_pokemon_data(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    let pokedex_number = match id.parse::<i32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let query_result = sqlx::query_file_as!(Pokemon, "src/sql/pokemon.sql", pokedex_number)
        .fetch_all(&pool)
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
    State(pool): State<PgPool>,
    Json(payload): Json<PokemonTypeQuery>,
) -> Result<String, (StatusCode, String)> {
    let query_result = sqlx::query_file_as!(
        PokemonTypeList,
        "src/sql/pokemonByType.sql",
        payload.type1,
        payload.type2
    )
    .fetch_all(&pool)
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
