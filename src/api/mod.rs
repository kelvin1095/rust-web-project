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
    test_route::list_things,
};

pub mod authorise;
pub mod pokemon_route;
pub mod register_user;
pub mod test_route;

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
        .with_state(connection);

    return app;
}
