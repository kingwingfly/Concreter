mod sym {
    tonic::include_proto!("sym");
}

use sym::sym_client::SymClient;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::config::config;

use super::error::RpcResult;

async fn sym_client() -> &'static SymClient<Channel> {
    static SYM_CLIENT: OnceCell<SymClient<Channel>> = OnceCell::const_new();
    SYM_CLIENT
        .get_or_init(|| async {
            let addr = config().SYM_ADDR.to_owned();
            let channel = Channel::from_shared(addr).unwrap().connect().await.unwrap();
            SymClient::new(channel)
        })
        .await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_sym_client() {
        let client = sym_client().await;
        println!("client: {:?}", client);
    }
}
