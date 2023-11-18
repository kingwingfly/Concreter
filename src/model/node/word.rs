use super::base::AgdbNodeBmc;
use crate::{impl_agdb_node_bmc, node_bmc_test};
use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
pub struct WordAg {
    db_id: Option<DbId>,
    pg_id: i64,
}

pub struct WordAgBmc;

impl_agdb_node_bmc!(WordAgBmc, WordAg, "words");

node_bmc_test!(WordAgBmc, WordAg, "words");
