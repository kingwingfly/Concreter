#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/index.html").await?.print().await?;

    let before_login_test = hc.do_get("/api/rpc");
    before_login_test.await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let after_login_test = hc.do_get("/api/rpc");
    after_login_test.await?.print().await?;

    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );
    req_logoff.await?.print().await?;

    // let register = hc.do_post(
    //     "/api/register",
    //     json!({
    //         "username": "demo2",
    //         "pwd": "welcome"
    //     }),
    // );
    // register.await?.print().await?;

    let req_article = hc.do_post(
        "/api/article",
        json!({
            "id": 1000
        }),
    );
    req_article.await?.print().await?;

    Ok(())
}
