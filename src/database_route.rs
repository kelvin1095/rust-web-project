use axum::{
    extract::{Path, State},
    http::StatusCode,
    //   routing::get,
    //   Router
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

// use std::time::Duration;
// use sqlx::postgres::PgPoolOptions;

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
    height: Option<f32>,
    weight: Option<f32>,
    pokemonimagefilename: Option<String>,
}

// pub async fn add_sql_route() -> Router {
//     // Specify db url for postgresql
//     let database_url = "postgresql://postgres:password@localhost/postgres";

//     // Set up postgresql connection pool
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .acquire_timeout(Duration::from_secs(3))
//         .connect(&database_url)
//         .await
//         .expect("can't connect to database");

//     return Router::new()
//         .route("/api/pokemon/:id", get(get_pokemon_data))
//         .with_state(pool);
// }

pub async fn get_pokemon_data(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    let query_result =
        sqlx::query_as::<_, Pokemon>("SELECT * FROM pokemon WHERE PokedexNumber = $1")
            .bind(id)
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
