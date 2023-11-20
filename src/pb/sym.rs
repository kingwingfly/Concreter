mod sym {
    tonic::include_proto!("sym");
}

use sym::sym_client::SymClient;
use sym::HelloRequest;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::config::config;

use super::error::RpcResult;

async fn sym_client() -> SymClient<Channel> {
    static SYM_CLIENT: OnceCell<SymClient<Channel>> = OnceCell::const_new();
    SYM_CLIENT
        .get_or_init(|| async {
            let addr = config().SYM_ADDR.to_owned();
            let channel = Channel::from_shared(addr).unwrap().connect().await.unwrap();
            SymClient::new(channel)
        })
        .await
        .clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_sym_client() {
        let mut client = sym_client().await;
        let req = HelloRequest {
            name: "Louis".to_string(),
        };
        let resp = client.say_hello(req).await.unwrap().into_inner();
        assert_eq!(resp.message, "Hello Louis!");
    }
}
