use agdb::{Comparison::Equal, Db, DbId, DbUserValue, QueryBuilder, QueryMut, UserValue};
use agdb::{Query, QueryResult};

use super::node::Node;
use super::DbResult;
use std::sync::RwLock;

/// Database back model controller
pub struct DbBmc {
    db: RwLock<Db>,
}

impl DbBmc {
    pub fn new() -> DbResult<Self> {
        #[cfg(test)]
        dotenv::dotenv().ok();

        let filename = std::env::var("DbFile")?;

        Ok(Self {
            db: RwLock::new(Db::new(&filename)?),
        })
    }

    fn exec<Q: Query>(&self, q: &Q) -> DbResult<QueryResult> {
        let db = self.db.read().unwrap();
        Ok(db.exec(q)?)
    }

    fn exec_mut<Q: QueryMut>(&self, q: &Q) -> DbResult<QueryResult> {
        let mut db = self.db.write().unwrap();
        Ok(db.exec_mut(q)?)
    }

    pub fn add_node<S: Into<String>>(&self, value: S) -> DbResult<Vec<DbId>> {
        let node = Node::new(value);
        let q = QueryBuilder::insert().nodes().values(&node).query();
        let db_ids = self.exec_mut(&q)?.ids();
        Ok(db_ids)
    }

    pub fn add_nodes<S: Into<String>>(&self, values: Vec<S>) -> DbResult<Vec<DbId>> {
        let nodes: Vec<Node> = values.into_iter().map(|name| Node::new(name)).collect();
        let q = QueryBuilder::insert().nodes().values(&nodes).query();
        let db_ids = self.exec_mut(&q)?;
        Ok(db_ids)
    }
}

#[cfg(test)]
mod db_tests {
    use super::*;

    #[test]
    fn create_test() {
        assert!(DbBmc::new().is_ok());
    }

    fn exec_test() {
        let mut db_bmc = DbBmc::new().unwrap();
    }
}
