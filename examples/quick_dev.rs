#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;
use time::convert;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // let before_login_test = hc.do_get("/api/rpc");
    // before_login_test.await?.print().await?;

    let req_list_ids = hc.do_get("/api/article/ids");
    req_list_ids.await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // let after_login_test = hc.do_get("/api/rpc");
    // after_login_test.await?.print().await?;

    // let req_logoff = hc.do_post(
    //     "/api/logoff",
    //     json!({
    //         "logoff": true
    //     }),
    // );
    // req_logoff.await?.print().await?;

    // let register = hc.do_post(
    //     "/api/register",
    //     json!({
    //         "username": "demo2",
    //         "pwd": "welcome"
    //     }),
    // );
    // register.await?.print().await?;

    // let req_article = hc.do_get("/api/article/1000");
    // req_article.await?.print().await?;

    let req_article_list = hc.do_get("/api/articles");
    req_article_list.await?.print().await?;

    // let req_upload = hc.do_post(
    //     "/api/article",
    //     json!({
    //         "title": "upload test",
    //         "content": "upload test",
    //         "field": "science"
    //     }),
    // );
    // req_upload.await?.print().await?;

    Ok(())
}
