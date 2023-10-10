use axum::{
    body::Bytes,
    // extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    Router
};
use tokio::signal;
use tower_http::{
    classify::ServerErrorsFailureClass,
    services::{ServeDir, ServeFile}, 
    trace::TraceLayer
};
use tracing::{info_span, Span};

use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .init();

    // Specify the IP address and port to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create an Axum application
    let mut app = Router::new();
    app = add_static_routes(app);
    app = add_tracing(app);
            
    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed to start");
}

fn add_static_routes(router: Router) -> Router {
    let serve_dir = ServeDir::new("build")
        .not_found_service(ServeFile::new("build/app.html"));

    return router
        .nest_service("/", serve_dir);
}

fn add_tracing(router: Router) -> Router {
    return router
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {

                // let matched_path = request
                //     .extensions()
                //     .get::<MatchedPath>()
                //     .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
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
        )
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