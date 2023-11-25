use axum::{extract::State, middleware::from_fn, routing::get, Router};
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{ModelManager, PgdbBmc, UserPg, UserPgBmc},
};

use super::{mw_auth::mw_ctx_require, AuthResult};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/user/name", get(api_user_name_get_handler))
        .with_state(mm)
        .route_layer(from_fn(mw_ctx_require))
}

async fn api_user_name_get_handler(ctx: AuthResult<Ctx>, State(mm): State<ModelManager>) -> String {
    debug!("{:<12} - api_username_get_handler", "HANDLER");
    let ctx = ctx.unwrap();
    let id = ctx.user_id();
    let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
    user.username
}
