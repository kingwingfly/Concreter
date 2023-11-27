use agdb::DbId;
use serde_json::Value;

use crate::{
    ctx::Ctx,
    model::{AgdbEdgeBmc, EntityAgBmc, EntityPgBmc, FormulaAgBmc, FormulaPgBmc, HasBmc},
};

use super::{
    AgdbNodeBmc, ArticleAg, ArticleAgBmc, ArticleNew, ArticlePg, ArticlePgBmc, DbResult, EntityAg,
    EntityNew, FormulaAg, FormulaNew, ModelManager, PgdbBmc,
};

pub struct ToStore {
    pub pg_id: i64,
    pub ag_id: DbId,
}

impl ToStore {
    pub async fn new(ctx: &Ctx, mm: &ModelManager, article: ArticleNew) -> DbResult<Self> {
        let pg_id = ArticlePgBmc::insert(ctx, mm, article).await?;
        let ag_id = ArticleAgBmc::update(
            ctx,
            mm,
            ArticleAg {
                db_id: None,
                pg_id,
                article: 0,
            },
        )
        .await?;
        Ok(Self { pg_id, ag_id })
    }

    pub async fn finish(&self, ctx: &Ctx, mm: &ModelManager) -> DbResult<()> {
        let origin: ArticlePg = ArticlePgBmc::first_by(ctx, mm, "id", self.pg_id).await?;
        ArticlePgBmc::update_one_field(ctx, mm, &origin, "status", "finished").await?;
        Ok(())
    }

    pub async fn add_entity(
        &self,
        ctx: &Ctx,
        mm: &ModelManager,
        name: String,
        attris: Value,
    ) -> DbResult<()> {
        let pg_id = EntityPgBmc::insert(ctx, mm, EntityNew { name, attris }).await?;
        let entity_ag_id = EntityAgBmc::update(
            ctx,
            mm,
            EntityAg {
                db_id: None,
                pg_id,
                entity: 0,
            },
        )
        .await?;
        HasBmc::connect(ctx, mm, self.ag_id, entity_ag_id).await?;
        Ok(())
    }

    pub async fn add_formula(
        &self,
        ctx: &Ctx,
        mm: &ModelManager,
        md: String,
        sym: String,
    ) -> DbResult<()> {
        let pg_id = FormulaPgBmc::insert(ctx, mm, FormulaNew { md, sym }).await?;
        let formula_ag_id = FormulaAgBmc::update(
            ctx,
            mm,
            FormulaAg {
                db_id: None,
                pg_id,
                formula: 0,
            },
        )
        .await?;
        HasBmc::connect(ctx, mm, self.ag_id, formula_ag_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::_dev_utils::{init_test, run_test};

    use super::*;
    use serde_json::json;

    #[test]
    fn store_test() {
        run_test(async {
            let ctx = Ctx::root_ctx();
            let mm = init_test().await;
            let to_store = ToStore::new(
                &ctx,
                &mm,
                ArticleNew {
                    author: 1000,
                    title: "hello".to_owned(),
                    content: "world".to_owned(),
                    field: "".to_owned(),
                },
            )
            .await
            .unwrap();
            let ag_id = to_store.ag_id;
            dbg!(ag_id);
            let json = json!({
                "name": {
                    "attr_name1": "attr1",
                    "attr_name2": {
                        "sub_attr_name1": "sub_attr1",
                        "sub_attr_name2": "sub_attr2"
                    }
                }
            });
            for (name, attris) in json.as_object().unwrap() {
                to_store
                    .add_entity(&ctx, &mm, name.to_owned(), attris.to_owned())
                    .await
                    .unwrap();
            }
            let entities = EntityAgBmc::get_next(&ctx, &mm, ag_id, "entity")
                .await
                .unwrap();
            dbg!(entities);
        })
    }
}
