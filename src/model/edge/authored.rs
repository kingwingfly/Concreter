use crate::model::base::AgdbEdgeBmc;
use crate::{edge_bmc_test, impl_agdb_edge_bmc};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct Authored {
    db_id: Option<DbId>,
    name: String,
}

pub struct AuthoredBmc;

impl_agdb_edge_bmc!(AuthoredBmc, Authored, "authored");

edge_bmc_test!(AuthoredBmc, User, UserBmc, Article, ArticleBmc);
