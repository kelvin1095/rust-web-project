use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod database_route;
// use crate::database_route::add_sql_route;
use crate::database_route::get_pokemon_data;

mod main_route;
use crate::main_route::add_static_routes;

// mod tracing;
// use crate::tracing::add_tracing_layer;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Specify the IP address and port to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Initialise subscriber that listens and prints logs
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    // Specify db url for postgresql
    let database_url = "postgresql://postgres:password@localhost/postgres";

    // Set up postgresql connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    // Create an Axum application
    let app = Router::new()
        .route("/api/pokemon/:id", get(get_pokemon_data))
        .with_state(pool)
        // .merge(add_sql_route_2())
        .merge(add_static_routes())
        .layer(TraceLayer::new_for_http());
    // .merge(add_tracing_layer());

    // let app = add_tracing_layer(app);

    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed to start");

    return Ok(());
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
