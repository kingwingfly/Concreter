use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    ctx::Ctx,
    model::{ArticlePg, ArticlePgBmc, ModelManager, PgdbBmc},
};

use super::ArticleResult;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/article", post(api_article_handler))
        .with_state(mm)
}

async fn api_article_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<GetArticelPayload>,
) -> ArticleResult<Json<ArticlePg>> {
    let root_ctx = Ctx::root_ctx();
    let GetArticelPayload { id } = payload;
    let article: ArticlePg = ArticlePgBmc::first_by(&root_ctx, &mm, "id", id).await?;
    let body = Json::from(article);
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct GetArticelPayload {
    id: i32,
}
