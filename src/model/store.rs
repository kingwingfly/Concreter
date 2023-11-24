use serde_json::Value;

use crate::{
    ctx::Ctx,
    model::{AgdbEdgeBmc, EntityAgBmc, EntityPgBmc, FormulaAgBmc, FormulaPgBmc, HasBmc},
};

use super::{
    AgdbNodeBmc, ArticleAg, ArticleAgBmc, ArticleNew, ArticlePgBmc, DbResult, EntityAg, EntityNew,
    FormulaAg, FormulaNew, ModelManager, PgdbBmc,
};

pub struct ToStore {
    article: ArticleNew,
    entities: Vec<EntityNew>,
    formulas: Vec<FormulaNew>,
}

impl ToStore {
    pub fn new(article: ArticleNew) -> Self {
        Self {
            article,
            entities: vec![],
            formulas: vec![],
        }
    }

    pub async fn store(self, ctx: &Ctx, mm: &ModelManager) -> DbResult<()> {
        let pg_id = ArticlePgBmc::insert(ctx, mm, self.article).await?;
        let article_ag_id = ArticleAgBmc::update(ctx, mm, ArticleAg { db_id: None, pg_id }).await?;
        for entity in self.entities {
            let pg_id = EntityPgBmc::insert(ctx, mm, entity).await?;
            let entity_ag_id =
                EntityAgBmc::update(ctx, mm, EntityAg { db_id: None, pg_id }).await?;
            HasBmc::connect(ctx, mm, article_ag_id, entity_ag_id).await?;
        }
        for formula in self.formulas {
            let pg_id = FormulaPgBmc::insert(ctx, mm, formula).await?;
            let formula_ag_id =
                FormulaAgBmc::update(ctx, mm, FormulaAg { db_id: None, pg_id }).await?;
            HasBmc::connect(ctx, mm, article_ag_id, formula_ag_id).await?;
        }
        Ok(())
    }

    pub fn add_entity(&mut self, name: String, attris: Value) {
        self.entities.push(EntityNew { name, attris })
    }

    pub fn add_formula(&mut self, md: String, sym: String) {
        self.formulas.push(FormulaNew { md, sym })
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
            let mut to_store = ToStore::new(ArticleNew {
                author: 1000,
                title: "hello".to_owned(),
                content: "world".to_owned(),
            });
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
                to_store.add_entity(name.to_owned(), attris.to_owned());
            }
            to_store.store(&ctx, &mm).await.unwrap();
        })
    }
}
