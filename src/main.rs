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

mod database_route;
use crate::database_route::get_pokemon_data;
use crate::database_route::get_pokemon_data_by_type;

mod main_route;
use crate::main_route::add_static_routes;

mod test_route;
use crate::test_route::list_things;

mod authorise;
use crate::authorise::authorize;

// mod tracing;
// use crate::tracing::add_tracing_layer;

#[tokio::main]
async fn main() {
    dotenv().expect("Missing dotenv file");

    // Specify the IP address and port to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Initialise subscriber that listens and prints logs
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    // Specify db url for postgresql
    let database_url: String = env::var("DATABASE_URL").expect("missing database url");
    // let database_url = "postgresql://postgres:password@localhost/postgres";

    // Set up postgresql connection pool
    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    // Create an Axum application
    let app = Router::new()
        .route("/api/pokemon/:id", get(get_pokemon_data))
        .route("/api/auth", post(authorize))
        .route("/api/pokemon/type", post(get_pokemon_data_by_type))
        .route("/api/sum", post(list_things))
        .with_state(pool)
        .merge(add_static_routes())
        .layer(TraceLayer::new_for_http());

    // let app = add_tracing_layer(app);

    // Start the Axum server
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
