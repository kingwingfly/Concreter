mod config;
mod ctx;
mod error;
mod model;
mod pb;
mod pwd;
mod token;
mod utils;
mod web;

mod _dev_utils;

use std::net::SocketAddr;

use axum::{
    middleware::{from_fn, from_fn_with_state, map_response},
    Router,
};
use error::*;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    model::ModelManager,
    web::{
        mw_auth::{mw_ctx_require, mw_ctx_resolve},
        mw_res_map::mw_reponse_map,
        routes_login, routes_static, rpc,
    },
};

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    let routes_rpc = rpc::routes(mm.clone()).route_layer(from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .nest("/api", routes_rpc)
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(from_fn_with_state(mm.clone(), mw_ctx_resolve))
                .layer(map_response(mw_reponse_map)),
        )
        .fallback_service(routes_static::serve_dir());

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
