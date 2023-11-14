use agdb::{DbId, UserValue};

use super::base::AgdbBmc;
use crate::{bmc_test, impl_agdbbmc};

#[derive(Debug, UserValue, Default)]
struct Word {
    db_id: Option<DbId>,
    content: String,
}

struct WordBmc;

impl_agdbbmc!(WordBmc, Word, "words");

bmc_test!(WordBmc, Word, "words");
