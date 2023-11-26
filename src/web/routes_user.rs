use axum::{extract::State, middleware::from_fn, routing::get, Json, Router};
use serde_json::{json, Value};
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{ModelManager, PgdbBmc, UserPg, UserPgBmc},
};

use super::{mw_auth::mw_ctx_require, AuthResult};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/user", get(api_user_get_handler))
        .with_state(mm)
        .route_layer(from_fn(mw_ctx_require))
}

async fn api_user_get_handler(ctx: AuthResult<Ctx>, State(mm): State<ModelManager>) -> Json<Value> {
    debug!("{:<12} - api_user_get_handler", "HANDLER");
    let ctx = ctx.unwrap();
    let id = ctx.user_id();
    let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
    Json(json!({"id": user.id, "username": user.username}))
}
