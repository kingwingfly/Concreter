use agdb::{DbId, UserValue};

use crate::model::base::AgdbNodeBmc;

use crate::{impl_agdb_node_bmc, node_bmc_test};

#[derive(Debug, UserValue, Default)]
pub struct Word {
    db_id: Option<DbId>,
    content: String,
}

pub struct WordBmc;

impl_agdb_node_bmc!(WordBmc, Word, "words");

node_bmc_test!(WordBmc, Word, "words");
