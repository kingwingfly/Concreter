use super::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct FormulaAg {
    db_id: Option<DbId>,
    pg_id: i64,
}

pub struct FormulaAgBmc;

impl_agdb_node_bmc!(FormulaAgBmc, FormulaAg, "formulas");

node_bmc_test!(FormulaAgBmc, FormulaAg, "formulas");
