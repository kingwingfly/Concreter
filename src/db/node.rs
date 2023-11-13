use agdb::{DbId, UserValue};

#[derive(Debug, UserValue)]
pub(super) struct Node {
    db_id: Option<DbId>,
    value: String,
}

impl Node {
    pub(super) fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            db_id: None,
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn create_test() {}
}
