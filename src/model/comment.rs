use agdb::{DbId, UserValue};

use super::base::AgdbBmc;
use crate::{bmc_test, impl_agdbbmc};

#[derive(Debug, UserValue, Default)]
struct Comment {
    db_id: Option<DbId>,
    content: String,
}

struct CommentBmc;

impl_agdbbmc!(CommentBmc, Comment, "comments");

bmc_test!(CommentBmc, Comment, "comments");
