use crate::model::Value;

use super::base::{Field, PgdbBmc};

use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct ArticlePg {
    pub id: i64,
    pub author: i64,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
}

impl Field for ArticlePg {
    fn pg_id(&self) -> i64 {
        self.id
    }

    fn values(&self) -> Vec<Value> {
        unreachable!()
    }

    fn keys(&self) -> Vec<String> {
        unreachable!()
    }
}

pub struct ArticleNew {
    pub author: i64,
    pub title: String,
    pub content: String,
}

impl Field for ArticleNew {
    fn pg_id(&self) -> i64 {
        unreachable!()
    }

    fn values(&self) -> Vec<Value> {
        vec![
            Value::Int(self.author),
            Value::String(self.title.to_owned()),
            Value::String(self.content.to_owned()),
        ]
    }

    fn keys(&self) -> Vec<String> {
        vec![
            "author".to_string(),
            "title".to_string(),
            "content".to_string(),
        ]
    }
}

pub struct ArticlePgBmc;

impl PgdbBmc for ArticlePgBmc {
    const TABLE: &'static str = "articles";
}

#[cfg(test)]
mod pg_tests {
    use crate::{
        _dev_utils::{init_test, run_test},
        ctx::Ctx,
    };

    use super::*;

    #[test]
    fn pg_test() {
        run_test(async {
            let ctx = Ctx::root_ctx();
            let mm = init_test().await;
            let article: ArticlePg = ArticlePgBmc::first_by(&ctx, &mm, "id", 1000 as i64)
                .await
                .unwrap();
            assert_eq!(article.author, 1000);
            assert_eq!(article.content, "hello world");
            ArticlePgBmc::update_one_field(&ctx, &mm, &article, "content", "hello louis")
                .await
                .unwrap();
            let article: ArticlePg = ArticlePgBmc::first_by(&ctx, &mm, "author", 1000 as i64)
                .await
                .unwrap();
            assert_eq!(article.content, "hello louis".to_string());
            let article = ArticleNew {
                author: 1000,
                title: "hello again".to_string(),
                content: "hello again".to_string(),
            };
            let id = ArticlePgBmc::insert(&ctx, &mm, article).await.unwrap();
            let article: ArticlePg = ArticlePgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
            assert_eq!(article.author, 1000);
            assert_eq!(article.content, "hello again");
            ArticlePgBmc::delete_by(&ctx, &mm, "id", id).await.unwrap();
            assert!(
                ArticlePgBmc::first_by::<ArticlePg, _, _>(&ctx, &mm, "id", id)
                    .await
                    .is_err()
            );
        })
    }
}
