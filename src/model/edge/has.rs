use agdb::{DbId, UserValue};

use crate::model::base::AgdbEdgeBmc;

use crate::{edge_bmc_test, impl_agdb_edge_bmc};

#[derive(Debug, UserValue, Default)]
pub struct Has {
    db_id: Option<DbId>,
    name: String,
}

pub struct HasBmc;

impl_agdb_edge_bmc!(HasBmc, Has, "has");

edge_bmc_test!(HasBmc, Article, ArticleBmc, Word, WordBmc);
