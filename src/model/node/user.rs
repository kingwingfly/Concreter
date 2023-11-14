use agdb::{DbId, UserValue};

use crate::model::base::AgdbNodeBmc;

use crate::{impl_agdb_node_bmc, node_bmc_test};

#[derive(Debug, UserValue, Default)]
pub struct User {
    db_id: Option<DbId>,
    content: String,
}

pub struct UserBmc;

impl_agdb_node_bmc!(UserBmc, User, "users");

node_bmc_test!(UserBmc, User, "users");
