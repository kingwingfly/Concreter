mod grpc_py;

use axum::{routing::get, Router};

use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
    // Is it possible to move this func in grpc::new_grpc_server() ?

    Router::new()
        .route("/rpc", get(|| async { "Hello, World!" }))
        // .nest_service("/greeter", grpc)
        .with_state(mm)
}
