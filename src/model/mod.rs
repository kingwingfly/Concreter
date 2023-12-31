mod base;
mod edge;
mod macros;
mod node;
mod pg;
mod store;

mod error;

pub use base::*;
pub use edge::*;
pub use error::*;
pub use node::*;
pub use pg::*;
pub use store::*;

use agdb::{Db, QueryBuilder, QueryError};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::config;

#[derive(Clone)]
pub struct ModelManager {
    agdb: Arc<RwLock<Db>>,
    pgdb: PgPool,
}

impl ModelManager {
    pub async fn new() -> DbResult<Self> {
        Ok(Self {
            agdb: Arc::new(RwLock::new(init_agdb()?)),
            pgdb: init_pgdb().await?,
        })
    }

    fn agdb(&self) -> &Arc<RwLock<Db>> {
        &self.agdb
    }

    fn pgdb(&self) -> &PgPool {
        &self.pgdb
    }
}

fn init_agdb() -> DbResult<Db> {
    let filename = &config().AG_FILE;

    // TODO auto create db file if not exists.
    let mut agdb = Db::new(filename)?;

    agdb.transaction_mut(|t| -> Result<(), QueryError> {
        for name in [
            "root", "articles", "users", "entities", "formulas", "comments",
        ] {
            match t.exec_mut(&QueryBuilder::insert().nodes().aliases(name).query()) {
                Ok(_) => {
                    t.exec_mut(&QueryBuilder::insert().edges().from("root").to(name).query())?;
                }
                Err(e) if e.description.contains("exists") => continue,
                Err(e) => Err(e)?,
            };
        }
        Ok(())
    })?;
    Ok(agdb)
}

async fn init_pgdb() -> DbResult<PgPool> {
    let url = &config().PG_URL;
    let pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(60))
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(4)
        .connect(url)
        .await?;
    Ok(pool)
}
