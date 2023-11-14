use agdb::{DbId, UserValue};

use crate::model::base::AgdbEdgeBmc;

use crate::{edge_bmc_test, impl_agdb_edge_bmc};

#[derive(Debug, UserValue, Default)]
pub struct Commented {
    db_id: Option<DbId>,
    name: String,
}

pub struct CommentedBmc;

impl_agdb_edge_bmc!(CommentedBmc, Commented, "commented");

edge_bmc_test!(CommentedBmc, User, UserBmc, Comment, CommentBmc);
