use super::base::AgdbEdgeBmc;
use crate::{edge_bmc_test, impl_agdb_edge_bmc};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct Has {
    db_id: Option<DbId>,
    has: u64,
}

pub struct HasBmc;

impl_agdb_edge_bmc!(HasBmc, Has, "has");

edge_bmc_test!(HasBmc, ArticleAg, ArticleAgBmc, EntityAg, EntityAgBmc);
