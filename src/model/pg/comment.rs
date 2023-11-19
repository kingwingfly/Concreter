use crate::model::Value;

use super::base::{Field, PgdbBmc};

use sqlx::FromRow;

#[derive(FromRow)]
pub struct CommentPg {
    pub id: i64,
    pub author: i64,
    pub content: String,
    pub article: i64,
    pub parent_comment: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
}

impl Field for CommentPg {
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

pub struct CommentNew {
    author: i64,
    content: String,
    article: i64,
    parent_comment: i64,
}

impl Field for CommentNew {
    fn pg_id(&self) -> i64 {
        unreachable!()
    }

    fn values(&self) -> Vec<Value> {
        vec![
            Value::Int(self.author),
            Value::String(self.content.to_owned()),
            Value::Int(self.article),
            Value::Int(self.parent_comment),
        ]
    }

    fn keys(&self) -> Vec<String> {
        vec![
            "author".to_string(),
            "content".to_string(),
            "article".to_string(),
            "parent_comment".to_string(),
        ]
    }
}

pub struct CommentPgBmc;

impl PgdbBmc for CommentPgBmc {
    const TABLE: &'static str = "comments";
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
            let comment: CommentPg = CommentPgBmc::first_by(&ctx, &mm, "id", 1 as i64)
                .await
                .unwrap();
            assert_eq!(comment.author, 1000);
            assert_eq!(comment.content, "hello world");
            CommentPgBmc::update_one_field(&ctx, &mm, comment, "content", "hello louis")
                .await
                .unwrap();
            let comment: CommentPg = CommentPgBmc::first_by(&ctx, &mm, "id", 1 as i64)
                .await
                .unwrap();
            assert_eq!(comment.content, "hello louis".to_string());
            let comment = CommentNew {
                author: 1000,
                content: "hello jake".to_string(),
                article: 1000,
                parent_comment: 1,
            };
            let id = CommentPgBmc::insert(&ctx, &mm, comment).await.unwrap();
            let comment: CommentPg = CommentPgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
            assert_eq!(comment.author, 1000);
            assert_eq!(comment.parent_comment, Some(1));
            CommentPgBmc::delete_by(&ctx, &mm, "id", id).await.unwrap();
            assert!(
                CommentPgBmc::first_by::<CommentPg, _, _>(&ctx, &mm, "id", id)
                    .await
                    .is_err()
            );
        })
    }
}
