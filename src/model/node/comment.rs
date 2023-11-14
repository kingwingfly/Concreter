use agdb::{DbId, UserValue};

use crate::model::base::AgdbNodeBmc;

use crate::{impl_agdb_node_bmc, node_bmc_test};

#[derive(Debug, UserValue, Default)]
pub struct Comment {
    db_id: Option<DbId>,
    content: String,
}

pub struct CommentBmc;

impl_agdb_node_bmc!(CommentBmc, Comment, "comments");

node_bmc_test!(CommentBmc, Comment, "comments");
