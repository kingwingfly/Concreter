use agdb::{DbId, UserValue};

use crate::model::base::AgdbNodeBmc;

use crate::{impl_agdb_node_bmc, node_bmc_test};

#[derive(Debug, UserValue, Default)]
pub struct Formula {
    db_id: Option<DbId>,
    content: String,
}

pub struct FormulaBmc;

impl_agdb_node_bmc!(FormulaBmc, Formula, "formulas");

node_bmc_test!(FormulaBmc, Formula, "formulas");
