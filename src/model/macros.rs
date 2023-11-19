/// impl AgdbNodeBmc
/// accept (bmc, node, alias)
#[macro_export]
macro_rules! impl_agdb_node_bmc {
    ($bmc: ident, $node: ident, $alias: expr) => {
        impl AgdbNodeBmc for $bmc {
            const ALIAS: &'static str = $alias;
            type Node = $node;
        }
    };
}

/// generate node test
/// accept (bmc, node, alias)
/// This will test create, update and get
#[macro_export]
macro_rules! node_bmc_test {
    ($bmc: ident, $node: ident, $alias: expr) => {
        #[cfg(test)]
        mod node_tests {
            use super::*;
            use $crate::_dev_utils::{init_test, run_test};
            use $crate::ctx::Ctx;

            #[test]
            fn node_test() {
                run_test(async {
                    let ctx = Ctx::root_ctx();
                    let mut mm = init_test().await;
                    let node = $node::default();
                    let db_id = $bmc::update(&ctx, &mut mm, node).await.unwrap();
                    let id = db_id.clone();
                    let mut node = $bmc::get(&ctx, &mm, db_id).await.unwrap();
                    assert_eq!(node.len(), 1);
                    assert_eq!(node[0].pg_id, 0);
                    let mut node = node.pop().unwrap();
                    node.pg_id = 1;
                    let db_id = $bmc::update(&ctx, &mut mm, node).await.unwrap();
                    assert_eq!(db_id, id);
                    let node = $bmc::get(&ctx, &mm, db_id).await.unwrap();
                    assert_eq!(node.len(), 1);
                    assert_eq!(node[0].pg_id, 1);
                })
            }
        }
    };
}

/// impl AgdbEdgeBmc
/// accept (bmc, edge, edge_name)
/// edge_name is one of the key of edge in graph database
#[macro_export]
macro_rules! impl_agdb_edge_bmc {
    ($bmc: ident, $edge: ident, $name: expr) => {
        impl AgdbEdgeBmc for $bmc {
            const EDGE_NAME: &'static str = $name;
        }
    };
}

/// generate edge test
/// accept (edge_bmc, from, from_bmc, to, to_bmc) for edge with from and to
/// or
/// accept (edge_bmc, self, self_bmc) for edge with self
#[macro_export]
macro_rules! edge_bmc_test {
    ($edge_bmc: ident, $self: ident, $self_bmc: ident) => {
        #[cfg(test)]
        mod edge_tests {
            use super::*;
            use $crate::_dev_utils::{init_test, run_test};
            use $crate::ctx::Ctx;
            use $crate::model::AgdbNodeBmc;
            use $crate::model::{$self, $self_bmc};

            #[test]
            fn edge_test() {
                run_test(async {
                    let ctx = Ctx::root_ctx();
                    let mut mm = init_test().await;
                    let node1 = $self::default();
                    let id1 = $self_bmc::update(&ctx, &mut mm, node1).await.unwrap();
                    let node2 = $self::default();
                    let id2 = $self_bmc::update(&ctx, &mut mm, node2).await.unwrap();
                    $edge_bmc::connect(&ctx, &mut mm, id1, id2).await.unwrap();
                })
            }
        }
    };

    ($edge_bmc: ident, $from: ident, $from_bmc: ident, $to: ident, $to_bmc: ident) => {
        #[cfg(test)]
        mod edge_tests {
            use super::*;
            use $crate::_dev_utils::{init_test, run_test};
            use $crate::ctx::Ctx;
            use $crate::model::AgdbNodeBmc;
            use $crate::model::{$from, $from_bmc, $to, $to_bmc};

            #[test]
            fn edge_test() {
                run_test(async {
                    let ctx = Ctx::root_ctx();
                    let mut mm = init_test().await;
                    let node1 = $from::default();
                    let id1 = $from_bmc::update(&ctx, &mut mm, node1).await.unwrap();
                    let node2 = $to::default();
                    let id2 = $to_bmc::update(&ctx, &mut mm, node2).await.unwrap();
                    $edge_bmc::connect(&ctx, &mut mm, id1, id2).await.unwrap();
                })
            }
        }
    };
}
