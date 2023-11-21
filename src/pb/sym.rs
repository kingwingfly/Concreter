mod sym {
    tonic::include_proto!("sym");
}

use sym::sym_client::SymClient;
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
    use sym::{ConvertMdRequest, HelloRequest, ValueType};

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
            symbols: "x y".into(),
            symbol: "".into(),
            value: "".into(),
            r#type: ValueType::Number.into(),
        };
        let resp = client.convert_md_formula(req).await.unwrap().into_inner();
        assert_eq!(resp.formula, "(x + y)**2");

        let req = ConvertMdRequest {
            md: "z = x^2 + 2xy + y^2".into(),
            symbols: "x y".into(),
            symbol: "x".into(),
            value: "2".into(),
            r#type: ValueType::Number.into(),
        };
        let resp = client.convert_md_formula(req).await.unwrap().into_inner();
        assert_eq!(resp.formula, "4.0*(0.5*y + 1)**2");
    }

    #[tokio::test]
    async fn sym_test2() {
        let mut client = sym_client().await;
        let req = ConvertMdRequest {
            md: "z = h (h - 0.5x)".into(),
            symbols: "h x".into(),
            symbol: "x".into(),
            value: "x=kh".into(),
            r#type: ValueType::Expr.into(),
        };
        let resp = client.convert_md_formula(req).await.unwrap().into_inner();
        assert_eq!(resp.formula, "-1.0*h**2*(0.5*k - 1.0)");
    }
}
