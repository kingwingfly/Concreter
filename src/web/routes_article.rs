use axum::{
    extract::{Path, State},
    middleware::from_fn,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::debug;

use crate::{
    anylize::{Analyzer, ArticleAnalyzer},
    ctx::Ctx,
    model::{
        AgdbNodeBmc, ArticleNew, ArticlePg, ArticlePgBmc, EntityAg, EntityAgBmc, ModelManager,
        PgdbBmc,
    },
};

use super::{mw_auth::mw_ctx_require, ArticleResult, AuthResult};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/article", post(api_article_anylize_handler))
        .route("/api/articles", get(api_article_list_handler))
        .route_layer(from_fn(mw_ctx_require))
        .route("/api/article/ids", get(api_article_ids_handler))
        .route("/api/article/:id", get(api_article_get_handler))
        .route("/api/article/entities", post(api_entities_ids_handler))
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

    let body = Json(article);
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

async fn api_article_ids_handler(State(mm): State<ModelManager>) -> Json<Vec<i64>> {
    debug!("{:<12} - api_article_ids_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let ids: Vec<i64> = ArticlePgBmc::list_all(&ctx, &mm, "id").await.unwrap();
    println!("{:?}", ids);
    let body = Json(ids);
    body
}

async fn api_article_list_handler(
    mm: State<ModelManager>,
    ctx: AuthResult<Ctx>,
) -> Json<Vec<ArticleInfo>> {
    debug!("{:<12} - api_article_list_handler", "HANDLER");
    let ctx = ctx.unwrap();
    let articles: Vec<ArticlePg> = ArticlePgBmc::list_by(&ctx, &mm, "author", ctx.user_id())
        .await
        .unwrap();
    let articles_info: Vec<ArticleInfo> = articles
        .into_iter()
        .map(|article| ArticleInfo {
            id: article.id,
            title: article.title,
            fragment: article.content.chars().take(100).collect(),
            field: article.field,
        })
        .collect();
    let body = Json(articles_info);
    body
}

#[derive(Debug, Serialize)]
struct ArticleInfo {
    id: i64,
    title: String,
    fragment: String,
    field: String,
}

async fn api_entities_ids_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<EntitiesPayload>,
) -> Json<Vec<i64>> {
    debug!("{:<12} - api_entities_ids_handler", "HANDLER");
    let EntitiesPayload { id } = payload;
    let ctx = Ctx::root_ctx();

    let ids: Vec<EntityAg> = EntityAgBmc::get_next(&ctx, &mm, id, "entity")
        .await
        .unwrap();
    let ids = ids.into_iter().map(|node| node.pg_id).collect();
    let body = Json(ids);
    body
}

#[derive(Debug, Deserialize)]
struct EntitiesPayload {
    id: i64,
}
