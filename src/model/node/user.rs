use super::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct UserAg {
    db_id: Option<DbId>,
    pg_id: i64,
    user: i64,
}

pub struct UserAgBmc;

impl_agdb_node_bmc!(UserAgBmc, UserAg, "users");

node_bmc_test!(UserAgBmc, UserAg, "users");
