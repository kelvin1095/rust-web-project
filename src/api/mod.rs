use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use crate::api::{
    authorise::authorize,
    pokemon_route::{get_pokemon_data, get_pokemon_data_by_type},
    register_user::new_user,
    sentance::sentence,
    test_route::list_things,
    test_route2::list_things2,
    word_list::word_list,
};

mod authorise;
mod pokemon_route;
mod register_user;
mod sentance;
mod test_route;
mod test_route2;
mod word_list;

pub struct AppState {
    pub connection_pool: Pool<Postgres>,
}

pub fn api_routes(pool: Pool<Postgres>) -> Router {
    let connection = Arc::new(AppState {
        connection_pool: pool,
    });

    let app = Router::new()
        .route("/api/auth", post(authorize))
        .route("/api/register", post(new_user))
        .route("/api/pokemon/type", post(get_pokemon_data_by_type))
        .route("/api/pokemon/:id", get(get_pokemon_data))
        .route("/api/sum", post(list_things))
        .route("/api/product", post(list_things2))
        .route("/api/wordlist", post(word_list))
        .route("/api/sentence", get(sentence))
        .with_state(connection);

    return app;
}
