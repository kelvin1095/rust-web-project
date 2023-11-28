use api::api_routes;
use axum::http::{header, HeaderValue};
use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};
use tracing::Level;

use std::env;
use std::net::SocketAddr;
use std::time::Duration;

mod api;

mod static_files;
use crate::static_files::serve_static_files;

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
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    let middleware = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("deny"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(30)));

    let app = Router::new()
        .merge(api_routes(pool))
        .merge(serve_static_files())
        .layer(middleware);

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
