mod base;
mod edge;
mod macros;
mod node;
mod pg;

mod error;

pub use base::*;
pub use edge::*;
pub use error::*;
pub use node::*;
pub use pg::*;

use agdb::{Db, QueryBuilder, QueryError};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::sync::RwLock;

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

    fn agdb(&self) -> Arc<RwLock<Db>> {
        self.agdb.clone()
    }

    fn pgdb(&self) -> &PgPool {
        &self.pgdb
    }
}

fn init_agdb() -> DbResult<Db> {
    #[cfg(test)]
    dotenv::dotenv().ok();

    let filename = std::env::var("AgdbFile")?;
    let mut agdb = Db::new(&filename)?;

    agdb.transaction_mut(|t| -> Result<(), QueryError> {
        for name in ["root", "articles", "users", "words", "formulas", "comments"] {
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
    #[cfg(test)]
    dotenv::dotenv().ok();

    let url = std::env::var("PgUrl")?;
    let pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(60))
        .acquire_timeout(std::time::Duration::from_secs(2))
        .max_connections(4)
        .connect(&url)
        .await?;
    Ok(pool)
}

#[cfg(test)]
mod create_mm_test {
    use super::*;

    #[test]
    fn agdb_init_test() {
        init_agdb().unwrap();
    }

    #[tokio::test]
    async fn pgdb_init_test() {
        init_pgdb().await.unwrap();
    }
}
