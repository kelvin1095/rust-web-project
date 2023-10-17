use axum::{
    body::Bytes,
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::signal;
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::TraceLayer,
};
use tracing::{
    // Level,
    Span
};
use std::net::SocketAddr;
use std::time::Duration;

mod database_route;
use crate::database_route::add_sql_route;

mod main_route;
use crate::main_route::add_static_routes;

// mod tracing;
// use crate::tracing::add_tracing_layer;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Specify the IP address and port to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Specify db url for postgresql
    let database_url = "postgresql://postgres:password@localhost/postgres";

    // Initialise subscriber that listens and prints logs
    tracing_subscriber::fmt()
        // .with_max_level(Level::DEBUG)
        .init();

    // Set up postgresql connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    // Create an Axum application
    let app = Router::new()
        .route("/api/pokemon/:id", get(add_sql_route))
        .with_state(pool)
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

    return Ok(())
}

fn add_tracing_layer(router: Router) -> Router {
    return router
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {

                // let matched_path = request
                //     .extensions()
                //     .get::<MatchedPath>()
                //     .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request ",
                    // method = ?request.method(),
                    uri = ?request.uri().path(),
                    // user_agent = ?request.headers().get("User-Agent"),
                    // matched_path,
                    // some_other_field = tracing::field::Empty,
                )

            })
            .on_request(|request: &Request<_>, _span: &Span| {
                tracing::info!("request: {} {}", request.method(), request.uri().path())
            })
            .on_response(|response: &Response, latency: Duration, _span: &Span| {
                tracing::info!("response: {} in {:?}", response.status(), latency)
            })
            .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                tracing::info!("sending {} bytes in {:?}", chunk.len(), latency)
            })
            .on_eos(|_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                tracing::info!("stream closed after {:?}", stream_duration)
            })
            .on_failure(|error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("error: {}", error)
            })
        );
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
