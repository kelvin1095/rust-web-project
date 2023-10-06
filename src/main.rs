use axum::{
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};
use tokio::signal;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Specify the IP address and port to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(using_serve_dir_with_assets_fallback().into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed to start");
}

fn using_serve_dir_with_assets_fallback() -> Router {
    // `ServeDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/doesnt-exist.jpg` will return `index.html`
    // rather than a 404
    let serve_dir = ServeDir::new("public")
        .not_found_service(ServeFile::new("public/index.html"));

    return Router::new()
        .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/public", serve_dir.clone())
        .fallback_service(serve_dir);
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