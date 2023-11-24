tonic::include_proto!("sym");

use sym_client::SymClient;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::config::config;

use super::error::RpcResult;

pub async fn sym_client() -> SymClient<Channel> {
    static SYM_CLIENT: OnceCell<SymClient<Channel>> = OnceCell::const_new();
    SYM_CLIENT
        .get_or_init(|| async {
            let addr = config().RPC_ADDR.to_owned();
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
    async fn hello_test() {
        let mut client = sym_client().await;
        let req = HelloRequest {
            name: "Louis".to_string(),
        };
        let resp = client.say_hello(req).await.unwrap().into_inner();
        assert_eq!(resp.message, "Hello Louis!");
    }

    #[tokio::test]
    async fn sym_test1() {
        let mut client = sym_client().await;
        let req = ConvertMdRequest {
            md: "z = x^2 + 2xy + y^2".into(),
        };
        let resp = client.convert_md_formula(req).await.unwrap().into_inner();
        println!("{}", resp.sym)
    }

    #[tokio::test]
    async fn sym_test2() {
        let mut client = sym_client().await;
        let req = ConvertMdRequest {
            md: "z = h (h - 0.5x)".into(),
        };
        let resp = client.convert_md_formula(req).await.unwrap().into_inner();
        println!("{}", resp.sym)
    }
}
