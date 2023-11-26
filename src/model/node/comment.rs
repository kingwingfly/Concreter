use super::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct CommentAg {
    db_id: Option<DbId>,
    pg_id: i64,
    comment: i64,
}

pub struct CommentAgBmc;

impl_agdb_node_bmc!(CommentAgBmc, CommentAg, "comments");

node_bmc_test!(CommentAgBmc, CommentAg, "comments");
