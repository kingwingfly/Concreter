use agdb::{DbId, UserValue};

use crate::model::base::AgdbEdgeBmc;

use crate::{edge_bmc_test, impl_agdb_edge_bmc};

#[derive(Debug, UserValue, Default)]
pub struct Rely {
    db_id: Option<DbId>,
    name: String,
}

pub struct RelyBmc;

impl_agdb_edge_bmc!(RelyBmc, Rely, "rely");

edge_bmc_test!(RelyBmc, Formula, FormulaBmc);
