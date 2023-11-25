use axum::{
    extract::{Path, State},
    middleware::from_fn,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing::debug;

use crate::{
    anylize::{Analyzer, ArticleAnalyzer},
    ctx::Ctx,
    model::{ArticleNew, ArticlePg, ArticlePgBmc, ModelManager, PgdbBmc},
};

use super::{mw_auth::mw_ctx_require, ArticleResult, AuthResult};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/article", post(api_article_anylize_handler))
        .route_layer(from_fn(mw_ctx_require))
        .route("/api/article/:id", get(api_article_get_handler))
        .with_state(mm)
}

async fn api_article_get_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> ArticleResult<Json<ArticlePg>> {
    debug!(
        "{:<12} - api_article_get_handler article id: {}",
        "HANDLER", id
    );
    let root_ctx = Ctx::root_ctx();
    let article: ArticlePg = ArticlePgBmc::first_by(&root_ctx, &mm, "id", id).await?;

    let body = Json::from(article);
    Ok(body)
}

async fn api_article_anylize_handler(
    State(mm): State<ModelManager>,
    ctx: AuthResult<Ctx>,
    Json(payload): Json<ArticleAnylizePayload>,
) -> ArticleResult<()> {
    debug!("{:<12} - api_article_anylize_handler", "HANDLER");
    let ctx = ctx.unwrap();
    let author = ctx.user_id();
    let ArticleAnylizePayload {
        title,
        content,
        field,
    } = payload;
    let new_article = ArticleNew {
        title,
        content,
        field,
        author,
    };
    tokio::spawn(async move {
        let _ = ArticleAnalyzer::analyze(&ctx, &mm, new_article).await;
    });
    Ok(())
}

#[derive(Debug, Deserialize)]
struct ArticleAnylizePayload {
    title: String,
    content: String,
    field: String,
}
