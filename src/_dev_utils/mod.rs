mod dev_db;

use lazy_static::lazy_static;
use tokio::sync::OnceCell;
use tracing::info;

pub fn test_rt() -> &'static tokio::runtime::Runtime {
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(init_test());
            rt
        };
    }
    &RT
}

pub fn run_test<F: std::future::Future>(f: F) -> F::Output {
    test_rt().block_on(f)
}

use crate::model::ModelManager;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INIT
        .get_or_init(|| async {
            println!("INIT ModelManager");
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;
    mm.clone()
}