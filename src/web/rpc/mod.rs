use axum::{
    routing::{get, post},
    Router,
};

use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", get(|| async { "Hello, World!" }))
        .with_state(mm)
}
