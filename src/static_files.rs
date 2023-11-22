use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn serve_static_files() -> Router {
    let serve_dir = ServeDir::new("build").fallback(ServeFile::new("build/app.html"));

    return Router::new().nest_service("/", serve_dir);
}
