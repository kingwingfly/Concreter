use super::base::AgdbBmc;
use crate::{bmc_test, impl_agdbbmc};

use agdb::{DbId, UserValue};

#[derive(Debug, UserValue, Default)]
struct Article {
    db_id: Option<DbId>,
    title: String,
    content: String,
}
struct ArticleBmc;

impl_agdbbmc!(ArticleBmc, Article, "articles");

bmc_test!(ArticleBmc, Article, "articles");
