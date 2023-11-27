use crate::config::config;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn routes() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new(&config().WEB_FOLDER)
            .not_found_service(ServeFile::new(&format!("{}/404.html", config().WEB_FOLDER))),
    )
}
