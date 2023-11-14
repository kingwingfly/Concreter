use crate::model::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};

use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct Article {
    db_id: Option<DbId>,
    title: String,
    content: String,
}
pub struct ArticleBmc;

impl_agdb_node_bmc!(ArticleBmc, Article, "articles");

node_bmc_test!(ArticleBmc, Article, "articles");
