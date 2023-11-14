use agdb::{DbId, UserValue};

use super::base::AgdbBmc;
use crate::{bmc_test, impl_agdbbmc};

#[derive(Debug, UserValue, Default)]
struct Formula {
    db_id: Option<DbId>,
    content: String,
}

struct FormulaBmc;

impl_agdbbmc!(FormulaBmc, Formula, "formulas");

bmc_test!(FormulaBmc, Formula, "formulas");
