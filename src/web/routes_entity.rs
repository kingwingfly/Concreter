use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{EntityPg, EntityPgBmc, ModelManager, PgdbBmc},
};

use super::EntityResult;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/entity/:id", get(api_entity_get_handler))
        .with_state(mm)
}

async fn api_entity_get_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> EntityResult<Json<EntityPg>> {
    debug!("{:<12} - api_entity_get_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let entity: EntityPg = EntityPgBmc::first_by(&ctx, &mm, "id", id).await?;
    let body = Json::from(entity);
    Ok(body)
}
