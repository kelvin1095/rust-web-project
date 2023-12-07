use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use crate::api::{
    authorise::authorize,
    logout::logout,
    mix_type_example::mixed_vector,
    pokemon_route::{get_pokemon_data, get_pokemon_data_by_type},
    register_user::new_user,
    sentence::sentence_list,
    test_route::list_things,
    word::word_list,
};

mod authorise;
mod check_jwt;
mod jwt_claims;
mod logout;
mod mix_type_example;
mod pokemon_route;
mod register_user;
mod sentence;
mod test_route;
mod word;

pub struct AppState {
    connection_pool: Pool<Postgres>,
    jwt_secret: String,
}

pub fn api_routes(pool: Pool<Postgres>) -> Router {
    let connection = Arc::new(AppState {
        connection_pool: pool,
        jwt_secret: std::env::var("JWT_SECRET_KEY").expect("missing secret key"),
    });

    let app = Router::new()
        .route("/api/auth", post(authorize))
        .route("/api/register", post(new_user))
        .route("/api/logout", post(logout))
        .route("/api/pokemon/type", post(get_pokemon_data_by_type))
        .route("/api/pokemon/:id", get(get_pokemon_data))
        .route("/api/sum", post(list_things))
        .route("/api/:language/word", post(word_list))
        .route("/api/:language/sentence", post(sentence_list))
        .route("/api/:language/example", get(mixed_vector))
        .with_state(connection);

    return app;
}
