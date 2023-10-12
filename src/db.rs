use axum::{
  extract::{State, Path},
  response::IntoResponse,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Pokemon {
    pokedexnumber: i32,
    name: String, 
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
    height: Option<String>,
    weight: Option<String>,
}

pub async fn add_sql_route(Path(id): Path<i32>, State(pool): State<PgPool>) -> impl IntoResponse {
  let query_result = sqlx::query_as::<_, Pokemon>("SELECT * FROM pokemon WHERE PokedexNumber = $1")
      .bind(id)
      .fetch_all(&pool)
      .await
      .expect("Failed to fetch data from the database");

  println!("sql fetched: {:?}", query_result);

  let json_result = serde_json::to_string_pretty(&query_result).unwrap();

  return (StatusCode::OK, json_result);
}