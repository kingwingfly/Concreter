use agdb::{DbId, DbUserValue, QueryBuilder, QueryError, QueryIds};

use crate::ctx::Ctx;

use super::{DbResult, ModelManager};

pub(super) trait AgdbBmc {
    const ALIAS: &'static str;
    type Node: DbUserValue;

    fn update<D>(_ctx: &Ctx, mm: &mut ModelManager, data: D) -> DbResult<DbId>
    where
        D: DbUserValue,
    {
        let db_id = mm
            .agdb_mut()
            .transaction_mut(|t| -> Result<DbId, QueryError> {
                match data.db_id() {
                    Some(id) => {
                        t.exec_mut(&QueryBuilder::insert().element(&data).query())?;
                        Ok(id)
                    }
                    None => {
                        let id = t
                            .exec_mut(
                                &QueryBuilder::insert()
                                    .nodes()
                                    .values(vec![data.to_db_values()])
                                    .query(),
                            )?
                            .ids()[0];
                        t.exec_mut(
                            &QueryBuilder::insert()
                                .edges()
                                .from(Self::ALIAS)
                                .to(id)
                                .query(),
                        )?;
                        Ok(id)
                    }
                }
            })?;
        Ok(db_id)
    }

    fn get<I>(_ctx: &Ctx, mm: &ModelManager, ids: I) -> DbResult<Vec<Self::Node>>
    where
        I: Into<QueryIds>,
    {
        let q = QueryBuilder::select().ids(ids).query();
        let node = mm.agdb().exec(&q)?.try_into()?;
        Ok(node)
    }
}

pub trait PgdbBmc {
    const TABLE: &'static str;
}