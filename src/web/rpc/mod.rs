mod sym;

use axum::{middleware::from_fn, routing::get, Router};

use crate::model::ModelManager;

use super::mw_auth::mw_ctx_require;

pub fn routes(mm: ModelManager) -> Router {
    // Is it possible to move this func in grpc::new_grpc_server() ?

    Router::new()
        .route("/rpc", get(|| async { "Hello, World!" }))
        // .nest_service("/greeter", grpc)
        .with_state(mm)
        .route_layer(from_fn(mw_ctx_require))
}
