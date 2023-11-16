use agdb::{DbId, DbUserValue, QueryBuilder, QueryError, QueryIds};

use crate::ctx::Ctx;

use super::{DbResult, ModelManager};

pub trait AgdbNodeBmc {
    const ALIAS: &'static str;
    type Node: DbUserValue;

    fn update<D>(_ctx: &Ctx, mm: &ModelManager, data: D) -> DbResult<DbId>
    where
        D: DbUserValue,
    {
        match data.db_id() {
            Some(id) => {
                mm.agdb()
                    .write()
                    .unwrap()
                    .exec_mut(&QueryBuilder::insert().element(&data).query())?;
                Ok(id)
            }

            None => {
                Ok(mm
                    .agdb()
                    .write()
                    .unwrap()
                    .transaction_mut(|t| -> Result<DbId, QueryError> {
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
                    })?)
            }
        }
    }

    fn get<I>(_ctx: &Ctx, mm: &ModelManager, ids: I) -> DbResult<Vec<Self::Node>>
    where
        I: Into<QueryIds>,
    {
        let q = QueryBuilder::select().ids(ids).query();
        let node = mm.agdb().read().unwrap().exec(&q)?.try_into()?;
        Ok(node)
    }
}

pub trait AgdbEdgeBmc {
    const EDGE_NAME: &'static str;

    fn connect<I>(_ctx: &Ctx, mm: &ModelManager, from: I, to: I) -> DbResult<()>
    where
        I: Into<QueryIds> + Copy,
    {
        mm.agdb()
            .write()
            .unwrap()
            .transaction_mut(|t| -> Result<(), QueryError> {
                t.exec_mut(
                    &QueryBuilder::insert()
                        .edges()
                        .from(from)
                        .to(to)
                        .values_uniform(vec![(Self::EDGE_NAME, 1).into()])
                        .query(),
                )?;
                Ok(())
            })?;
        Ok(())
    }
}

pub trait PgdbBmc {
    const TABLE: &'static str;
}
