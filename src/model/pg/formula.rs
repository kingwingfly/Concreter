use crate::model::Value;

use super::base::{Field, PgdbBmc};

use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct FormulaPg {
    pub id: i64,
    pub md: String,
    pub sym: String,
}

impl Field for FormulaPg {
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

pub struct FormulaNew {
    pub md: String,
    pub sym: String,
}

impl Field for FormulaNew {
    fn pg_id(&self) -> i64 {
        unreachable!()
    }

    fn values(&self) -> Vec<Value> {
        vec![
            Value::String(self.md.to_owned()),
            Value::String(self.sym.to_owned()),
        ]
    }

    fn keys(&self) -> Vec<String> {
        vec!["md".to_string(), "sym".to_string()]
    }
}

pub struct FormulaPgBmc;

impl PgdbBmc for FormulaPgBmc {
    const TABLE: &'static str = "formulas";
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
            let formula: FormulaPg = FormulaPgBmc::first_by(&ctx, &mm, "id", 1 as i64)
                .await
                .unwrap();
            assert_eq!(formula.md, "y = x ^ 2");
            FormulaPgBmc::update_one_field(&ctx, &mm, &formula, "md", "y = x ^ 3")
                .await
                .unwrap();
            let formula: FormulaPg = FormulaPgBmc::first_by(&ctx, &mm, "md", "y = x ^ 3")
                .await
                .unwrap();
            assert_eq!(formula.md, "y = x ^ 3");
            let formula = FormulaNew {
                md: "y = (x+1) ^ 2".to_string(),
                sym: "def formula() -> ...".to_string(),
            };
            let id = FormulaPgBmc::insert(&ctx, &mm, formula).await.unwrap();
            let formula: FormulaPg = FormulaPgBmc::first_by(&ctx, &mm, "id", id).await.unwrap();
            assert_eq!(formula.md, "y = (x+1) ^ 2");
            FormulaPgBmc::delete_by(&ctx, &mm, "id", id).await.unwrap();
            assert!(
                FormulaPgBmc::first_by::<FormulaPg, _, _>(&ctx, &mm, "id", id)
                    .await
                    .is_err()
            );
        })
    }
}
