use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn add_static_routes() -> Router {
    let serve_dir = ServeDir::new("build").not_found_service(ServeFile::new("build/app.html"));

    return Router::new().nest_service("/", serve_dir);
}
