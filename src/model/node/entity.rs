use super::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct EntityAg {
    pub db_id: Option<DbId>,
    pub pg_id: i64,
}

pub struct EntityAgBmc;

impl_agdb_node_bmc!(EntityAgBmc, EntityAg, "entities");

node_bmc_test!(EntityAgBmc, EntityAg, "entities");
