use super::base::AgdbEdgeBmc;
use crate::{edge_bmc_test, impl_agdb_edge_bmc};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct Rely {
    db_id: Option<DbId>,
    rely: u64,
}

pub struct RelyBmc;

impl_agdb_edge_bmc!(RelyBmc, Rely, "rely");

edge_bmc_test!(RelyBmc, FormulaAg, FormulaAgBmc);
