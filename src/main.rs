use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::Level;

use std::env;
use std::net::SocketAddr;
use std::time::Duration;

mod api;
use crate::api::{
    authorise::authorize,
    pokemon_route::{get_pokemon_data, get_pokemon_data_by_type},
    register_user::new_user,
};

mod static_files;
use crate::static_files::serve_static_files;

mod test_route;
use crate::test_route::list_things;

// mod tracing;
// use crate::tracing::add_tracing_layer;

#[tokio::main]
async fn main() {
    dotenv().expect("Missing dotenv file");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let database_url: String = env::var("DATABASE_URL").expect("missing database url");

    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    let app = Router::new()
        .route("/api/auth", post(authorize))
        .route("/api/register", post(new_user))
        .route("/api/pokemon/type", post(get_pokemon_data_by_type))
        .route("/api/pokemon/:id", get(get_pokemon_data))
        .route("/api/sum", post(list_things))
        .with_state(pool)
        .merge(serve_static_files())
        .layer(TraceLayer::new_for_http());

    // let app = add_tracing_layer(app);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed to start");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
