use std::fmt::Display;

use agdb::{DbId, DbUserValue, QueryBuilder, QueryError, QueryIds};
use sqlx::{postgres::PgRow, Encode, FromRow, Postgres, Row, Type};

use crate::ctx::Ctx;

use super::{DbResult, ModelManager};

pub trait AgdbNodeBmc {
    const ALIAS: &'static str;
    type Node: DbUserValue;

    async fn update<D>(_ctx: &Ctx, mm: &ModelManager, data: D) -> DbResult<DbId>
    where
        D: DbUserValue,
    {
        match data.db_id() {
            Some(id) => {
                mm.agdb()
                    .write()
                    .await
                    .exec_mut(&QueryBuilder::insert().element(&data).query())?;
                Ok(id)
            }
            None => {
                Ok(mm
                    .agdb()
                    .write()
                    .await
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

    async fn get<I>(_ctx: &Ctx, mm: &ModelManager, ids: I) -> DbResult<Vec<Self::Node>>
    where
        I: Into<QueryIds>,
    {
        let q = QueryBuilder::select().ids(ids).query();
        let node = mm.agdb().read().await.exec(&q)?.try_into()?;
        Ok(node)
    }
}

pub trait AgdbEdgeBmc {
    const EDGE_NAME: &'static str;

    async fn connect<I>(_ctx: &Ctx, mm: &ModelManager, from: I, to: I) -> DbResult<()>
    where
        I: Into<QueryIds> + Copy,
    {
        mm.agdb()
            .write()
            .await
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
    /// The table name in the database which changes will perform on
    const TABLE: &'static str;

    async fn insert<D>(_ctx: &Ctx, mm: &ModelManager, data: D) -> DbResult<i64>
    where
        D: Field,
    {
        let keys = data.keys();
        let mut sql = format!("INSERT INTO {} (", Self::TABLE);
        sql.push_str(&keys.join(","));
        sql.push_str(") VALUES (");
        sql.push_str(
            &(1..=keys.len())
                .map(|i| format!("${i}"))
                .collect::<Vec<String>>()
                .join(","),
        );
        sql.push_str(") RETURNING id");
        let mut q = sqlx::query(&sql);
        for v in data.values() {
            match v {
                Value::Int(v) => q = q.bind(v),
                Value::String(v) => q = q.bind(v),
            }
        }
        let id = q.fetch_one(mm.pgdb()).await?.try_get(0)?;
        Ok(id)
    }

    /// Return the first row of the table matching the field=value
    async fn first_by<D, F, V>(_ctx: &Ctx, mm: &ModelManager, field: F, value: V) -> DbResult<D>
    where
        for<'r> D: FromRow<'r, PgRow> + Send + Sync + Unpin,
        F: AsRef<str>,
        for<'q> V: Send + Encode<'q, Postgres> + Type<Postgres>,
    {
        Ok(sqlx::query_as(&format!(
            "SELECT * FROM {} WHERE {} = $1 LIMIT 1",
            Self::TABLE,
            field.as_ref()
        ))
        .bind(value)
        .fetch_one(mm.pgdb())
        .await?)
    }

    /// Delete the rows matching the field=value
    async fn delete_by<F, V>(_ctx: &Ctx, mm: &ModelManager, field: F, value: V) -> DbResult<()>
    where
        F: AsRef<str>,
        for<'q> V: Send + Encode<'q, Postgres> + Type<Postgres>,
    {
        sqlx::query(&format!(
            "DELETE FROM {} WHERE {} = $1",
            Self::TABLE,
            field.as_ref()
        ))
        .bind(value)
        .execute(mm.pgdb())
        .await?;
        Ok(())
    }

    /// Update the rows according to origin's id, and only update field to new_value
    async fn update_one_field<D, F, V>(
        _ctx: &Ctx,
        mm: &ModelManager,
        origin: D,
        field: F,
        new_value: V,
    ) -> DbResult<i64>
    where
        D: Field,
        F: AsRef<str>,
        for<'q> V: Send + Encode<'q, Postgres> + Type<Postgres>,
    {
        let id: i64 = sqlx::query(&format!(
            "UPDATE {} SET {}=$1 WHERE id=$2 RETURNING id",
            Self::TABLE,
            field.as_ref()
        ))
        .bind(new_value)
        .bind(origin.pg_id())
        .fetch_one(mm.pgdb())
        .await?
        .try_get(0)?;
        Ok(id)
    }
}

pub enum Value {
    Int(i64),
    String(String),
}

pub trait Field {
    /// Return the field names, which will be used in SQL query when insert.
    fn keys(&self) -> Vec<String>;
    /// Return the field values, which matched the keys.
    fn values(&self) -> Vec<Value>;
    /// Return the id of the row in the database
    fn pg_id(&self) -> i64;
}
