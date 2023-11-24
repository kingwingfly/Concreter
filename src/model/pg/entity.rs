use crate::model::Value;

use super::base::{Field, PgdbBmc};

use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct EntityPg {
    pub id: i64,
    pub name: String,
    pub attris: serde_json::Value,
}

impl Field for EntityPg {
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

pub struct EntityNew {
    pub name: String,
    pub attris: serde_json::Value,
}

impl Field for EntityNew {
    fn pg_id(&self) -> i64 {
        unreachable!()
    }

    fn values(&self) -> Vec<Value> {
        vec![
            Value::String(self.name.to_owned()),
            Value::Json(self.attris.to_owned()),
        ]
    }

    fn keys(&self) -> Vec<String> {
        vec!["name".to_string(), "attris".to_string()]
    }
}

pub struct EntityPgBmc;

impl PgdbBmc for EntityPgBmc {
    const TABLE: &'static str = "entities";
}

#[cfg(test)]
mod pg_tests {
    use serde_json::json;

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
            let entity: EntityPg = EntityPgBmc::first_by(&ctx, &mm, "id", 1 as i64)
                .await
                .unwrap();
            assert_eq!(entity.name, "Genshin Impact");
            assert_eq!(entity.attris["alias"], "原神");
            EntityPgBmc::update_one_field(&ctx, &mm, &entity, "name", "原神")
                .await
                .unwrap();
            let entity: EntityPg = EntityPgBmc::first_by(&ctx, &mm, "name", "原神")
                .await
                .unwrap();
            assert_eq!(entity.attris["alias"], "原神");
            let entity = EntityNew {
                name: "芙宁娜".to_string(),
                attris: json!({"alias": "芙芙", "country": "枫丹"}),
            };
            let id = EntityPgBmc::insert(&ctx, &mm, entity).await.unwrap();
            let entity: EntityPg = EntityPgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
            assert_eq!(entity.name, "芙宁娜");
            EntityPgBmc::delete_by(&ctx, &mm, "id", id).await.unwrap();
            assert!(EntityPgBmc::first_by::<EntityPg, _, _>(&ctx, &mm, "id", id)
                .await
                .is_err());
        })
    }
}
