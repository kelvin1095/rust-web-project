use axum::{
    extract::Query,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::Level;

use serde::{Deserialize, Serialize};

mod database_route;
use crate::database_route::get_pokemon_data;
use crate::database_route::get_pokemon_data_by_type;

mod main_route;
use crate::main_route::add_static_routes;

// mod tracing;
// use crate::tracing::add_tracing_layer;

#[derive(Deserialize)]
struct Summation {
    num1: String,
    num2: String,
}

#[derive(Serialize)]
struct Total {
    total: f32,
    method: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
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
        .min_connections(2)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    // Create an Axum application
    let app = Router::new()
        .route("/api/pokemon/:id", get(get_pokemon_data))
        .route("/api/pokemon/type", post(get_pokemon_data_by_type))
        .route("/api/sum", get(list_things).post(list_things_2))
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

    return Ok(());
}

async fn list_things(summation: Query<Summation>) -> Result<String, (StatusCode, String)> {
    let numbers: Summation = summation.0;

    let num1_parsed = match numbers.num1.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let num2_parsed = match numbers.num2.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let total: Total = Total {
        total: num1_parsed + num2_parsed,
        method: "GET".to_string(),
    };

    let result = serde_json::to_string_pretty(&total);

    return match result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}

async fn list_things_2(Json(payload): Json<Summation>) -> Result<String, (StatusCode, String)> {
    let num1_parsed = match payload.num1.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let num2_parsed = match payload.num2.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let total: Total = Total {
        total: num1_parsed + num2_parsed,
        method: "Post".to_string(),
    };

    let result = serde_json::to_string_pretty(&total);

    return match result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
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
