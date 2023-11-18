use super::base::AgdbEdgeBmc;
use crate::{edge_bmc_test, impl_agdb_edge_bmc};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct Commented {
    db_id: Option<DbId>,
    commented: u64,
}

pub struct CommentedBmc;

impl_agdb_edge_bmc!(CommentedBmc, Commented, "commented");

edge_bmc_test!(CommentedBmc, UserAg, UserAgBmc, CommentAg, CommentAgBmc);
