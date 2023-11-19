use crate::model::Value;

use super::base::{Field, PgdbBmc};

use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserPg {
    pub id: i64,
    pub username: String,
    pub pwd: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

impl Field for UserPg {
    fn pg_id(&self) -> i64 {
        self.id
    }

    fn values<'a>(&self) -> Vec<Value> {
        unreachable!()
    }

    fn keys(&self) -> Vec<String> {
        unreachable!()
    }
}

pub struct UserPgNew {
    username: String,
    pwd: String,
}

impl Field for UserPgNew {
    fn pg_id(&self) -> i64 {
        unreachable!()
    }

    fn values(&self) -> Vec<Value> {
        vec![
            Value::String(self.username.to_owned()),
            Value::String(self.pwd.to_owned()),
        ]
    }

    fn keys(&self) -> Vec<String> {
        vec!["username".to_string(), "pwd".to_string()]
    }
}

pub struct UserPgBmc;

impl PgdbBmc for UserPgBmc {
    const TABLE: &'static str = "users";
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
            let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "username", "demo1")
                .await
                .unwrap();
            assert_eq!(user.username, "demo1");
            assert_eq!(user.id, 1000);
            UserPgBmc::update_one_field(&ctx, &mm, user, "pwd", "123456")
                .await
                .unwrap();
            let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "username", "demo1")
                .await
                .unwrap();
            assert_eq!(user.pwd, Some("123456".to_string()));
            let user = UserPgNew {
                username: "louis".to_string(),
                pwd: "123456".to_string(),
            };
            let id = UserPgBmc::insert(&ctx, &mm, user).await.unwrap();
            let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
            assert_eq!(user.username, "louis");
            UserPgBmc::delete_by(&ctx, &mm, "id", id).await.unwrap();
            assert!(UserPgBmc::first_by::<UserPg, _, _>(&ctx, &mm, "id", id)
                .await
                .is_err());
        });
    }
}
