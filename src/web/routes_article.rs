use crate::{
    analyze::{Analyzer, ArticleAnalyzer},
    ctx::Ctx,
    model::{
        AgdbNodeBmc, ArticleAgBmc, ArticleNew, ArticlePg, ArticlePgBmc, EntityAg, EntityAgBmc,
        ModelManager, PgdbBmc,
    },
};
use axum::{
    extract::{Multipart, Path, State},
    middleware::from_fn,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tracing::debug;

use super::{mw_auth::mw_ctx_require, ArticleResult, AuthResult};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/article", post(api_article_anylize_handler))
        .route("/api/articles", get(api_article_list_handler))
        .route_layer(from_fn(mw_ctx_require))
        .route("/api/article/ids", get(api_article_ids_handler))
        .route("/api/article/:id", get(api_article_get_handler))
        .route("/api/article/:id/entities", get(api_entity_ids_handler))
        .route("/api/article/:id/formulas", get(api_formula_ids_handler))
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
    mut multipart: Multipart,
) -> ArticleResult<()> {
    debug!("{:<12} - api_article_anylize_handler", "HANDLER");
    let (mut filename, mut content, mut field) = (None, None, None);
    while let Some(mut field_) = multipart.next_field().await.unwrap() {
        let name = field_.name().unwrap().to_string();
        let data = field_.bytes().await.unwrap();
        let s = String::from_utf8(data.to_vec()).unwrap();
        match name.as_str() {
            "filename" => filename = Some(s),
            "content" => content = Some(s),
            "field" => field = Some(s),
            _ => {}
        }
    }
    let ctx = ctx.unwrap();
    let author = ctx.user_id();
    if let (Some(filename), Some(content), Some(field)) = (filename, content, field) {
        let new_article = ArticleNew {
            title: filename,
            content,
            field,
            author,
        };
        tokio::spawn(async move {
            let _ = ArticleAnalyzer::analyze(&ctx, &mm, new_article).await;
        });
    }
    Ok(())
}

async fn api_article_ids_handler(State(mm): State<ModelManager>) -> Json<Vec<i64>> {
    debug!("{:<12} - api_article_ids_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let ids: Vec<i64> = ArticlePgBmc::list_all(&ctx, &mm, "id").await.unwrap();
    println!("{:?}", ids);
    Json(ids)
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
            fragment: match article.status.as_str() {
                "finished" => article.content.chars().take(100).collect(),
                s => s.to_owned(),
            },
            field: article.field,
        })
        .collect();
    Json(articles_info)
}

#[derive(Debug, Serialize)]
struct ArticleInfo {
    id: i64,
    title: String,
    fragment: String,
    field: String,
}

async fn api_entity_ids_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> ArticleResult<Json<Vec<i64>>> {
    debug!("{:<12} - api_entities_ids_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let id = ArticleAgBmc::convert_pg_to_ag(&ctx, &mm, id).await?;

    let ids: Vec<EntityAg> = EntityAgBmc::get_next(&ctx, &mm, id, "entity")
        .await
        .unwrap();
    let ids = ids.into_iter().map(|node| node.pg_id).collect();
    let body = Json(ids);
    Ok(body)
}

async fn api_formula_ids_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> ArticleResult<Json<Vec<i64>>> {
    debug!("{:<12} - api_entities_ids_handler", "HANDLER");
    let ctx = Ctx::root_ctx();
    let id = ArticleAgBmc::convert_pg_to_ag(&ctx, &mm, id).await?;

    let ids: Vec<EntityAg> = EntityAgBmc::get_next(&ctx, &mm, id, "formula")
        .await
        .unwrap();
    let ids = ids.into_iter().map(|node| node.pg_id).collect();
    let body = Json(ids);
    Ok(body)
}
