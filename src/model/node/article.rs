use super::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct ArticleAg {
    pub db_id: Option<DbId>,
    pub pg_id: i64,
    pub article: i64,
}

pub struct ArticleAgBmc;

impl_agdb_node_bmc!(ArticleAgBmc, ArticleAg, "articles");

node_bmc_test!(ArticleAgBmc, ArticleAg, "articles");
