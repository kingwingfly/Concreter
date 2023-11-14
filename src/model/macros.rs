#[macro_export]
macro_rules! impl_agdbbmc {
    ($bmc: ident, $node: ident, $alias: expr) => {
        impl AgdbBmc for $bmc {
            const ALIAS: &'static str = $alias;
            type Node = $node;
        }
    };
}

#[macro_export]
macro_rules! bmc_test {
    ($bmc: ident, $node: ident, $alias: expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::ctx::Ctx;
            use $crate::model::ModelManager;

            #[test]
            fn update_test() {
                let ctx = Ctx::root_user();
                let mut mm = ModelManager::new().unwrap();
                let node = $node::default();
                let db_id = $bmc::update(&ctx, &mut mm, node).unwrap();
                let id = db_id.clone();
                let mut node = $bmc::get(&ctx, &mm, db_id).unwrap();
                assert_eq!(node.len(), 1);
                assert_eq!(node[0].content, "");
                let mut node = node.pop().unwrap();
                node.content = "hello".to_string();
                let db_id = $bmc::update(&ctx, &mut mm, node).unwrap();
                assert_eq!(db_id, id);
                let node = $bmc::get(&ctx, &mm, db_id).unwrap();
                assert_eq!(node.len(), 1);
                assert_eq!(node[0].content, "hello");
            }
        }
    };
}
