use agdb::{DbId, UserValue};

use super::base::AgdbBmc;
use crate::{bmc_test, impl_agdbbmc};

#[derive(Debug, UserValue, Default)]
struct User {
    db_id: Option<DbId>,
    content: String,
}

struct UserBmc;

impl_agdbbmc!(UserBmc, User, "users");

bmc_test!(UserBmc, User, "users");
