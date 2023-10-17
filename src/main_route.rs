use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_status::SetStatus;

pub fn add_static_routes() -> ServeDir<SetStatus<ServeFile>> {
    let serve_dir = ServeDir::new("build")
        .not_found_service(ServeFile::new("build/app.html"));

    return serve_dir;
}