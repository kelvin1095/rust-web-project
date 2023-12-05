use api::api_routes;
use axum::{
    http::{header, HeaderValue},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{set_header::SetResponseHeaderLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::Level;

use std::{env, time::Duration};

mod api;
mod static_files;
use crate::static_files::serve_static_files;

#[tokio::main]
async fn main() {
    dotenv().expect("Missing dotenv file");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("missing database url");

    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
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
        .layer(TimeoutLayer::new(Duration::from_secs(2)));

    let app = Router::new()
        .merge(api_routes(pool))
        .merge(serve_static_files())
        .layer(middleware);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
