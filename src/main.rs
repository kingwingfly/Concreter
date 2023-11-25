mod anylize;
mod config;
mod ctx;
mod error;
mod model;
mod nlp;
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
        routes_article, routes_entity, routes_formula, routes_login, routes_static, routes_user,
        rpc,
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

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .merge(routes_article::routes(mm.clone()))
        .merge(routes_entity::routes(mm.clone()))
        .merge(routes_formula::routes(mm.clone()))
        .merge(routes_user::routes(mm.clone()))
        .nest("/api", rpc::routes(mm.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(from_fn_with_state(mm.clone(), mw_ctx_resolve))
                .layer(map_response(mw_reponse_map)),
        )
        .merge(routes_static::routes());

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
