use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::ctx::Ctx;
use crate::model::{ArticlePg, ArticlePgBmc, ModelManager, PgdbBmc};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/md/:id", get(md_handler))
        .with_state(mm)
}

async fn md_handler(State(mm): State<ModelManager>, Path(id): Path<i64>) -> impl IntoResponse {
    let ctx = Ctx::root_ctx();
    let article: ArticlePg = ArticlePgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
    article.content
}
