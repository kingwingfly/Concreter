use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{FormulaPg, FormulaPgBmc, ModelManager, PgdbBmc},
};

use super::EntityResult;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/formula/:id", get(api_formula_get_handler))
        .with_state(mm)
}

async fn api_formula_get_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> EntityResult<Json<FormulaPg>> {
    debug!("{:<12} - api_formula_get_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let formula: FormulaPg = FormulaPgBmc::first_by(&ctx, &mm, "id", id).await?;
    let body = Json::from(formula);
    Ok(body)
}
