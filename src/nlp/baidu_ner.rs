use snafu::ResultExt;

use super::{ner_error, NerResult};
use crate::config::config;
use reqwest::header::{self, HeaderMap, HeaderValue};

pub struct Ner {
    client: reqwest::Client,
}

impl Ner {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        Ner {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn ner(&self, text: String) -> NerResult<NerRet> {
        let url = format!(
            "https://aip.baidubce.com/rpc/2.0/nlp/v1/entity_analysis?access_token={}",
            self.get_access_token().await?
        );
        let resp = self
            .client
            .post(url)
            .json(&Text {
                text: text.to_owned(),
            })
            .send()
            .await
            .context(ner_error::FaildNer {
                text: text.to_owned(),
            })?;
        // let ret = resp.text().await.unwrap();
        // println!("{}", ret);
        let ret: NerRet = resp
            .json()
            .await
            .map_err(|_| ner_error::DecodeError.build())?;
        Ok(ret)
    }

    async fn get_access_token(&self) -> NerResult<String> {
        let url = format!("https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id={}&client_secret={}",
            &config().API_KEY,&config().SECRET_KEY).parse::<reqwest::Url>().unwrap();
        let resp = self
            .client
            .post(url)
            .send()
            .await
            .map_err(|_| ner_error::GetAccessError.build())?;
        let ret = resp
            .json::<Keys>()
            .await
            .map_err(|_| ner_error::GetAccessError.build())?;
        Ok(ret.access_token)
    }
}

#[derive(serde::Serialize)]
struct Text {
    text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NerRet {
    text: String,
    entity_analysis: Vec<Analysis>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Analysis {
    mention: String,
    category: Category,
    confidence: f64,
    desc: String,
    status: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Category {
    level_1: String,
    level_2: String,
    level_3: String,
}

#[derive(Debug, serde::Deserialize)]
struct Keys {
    access_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Read};

    #[tokio::test]
    async fn baidu_ner_test() {
        let ner = Ner::new();
        let mut f = fs::File::open("examples/建筑工程合同管理.md").unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        let chunk_size = 96;
        let mut i = 0;
        for text in buffer.chars().collect::<Vec<_>>().chunks(chunk_size) {
            i += 1;
            if i < 3 {
                continue;
            }
            let text: String = text.into_iter().collect();
            println!("{}", text);
            let ret = ner.ner(text).await.unwrap();
            dbg!(&ret);
            if i == 3 {
                break;
            }
        }
    }

    #[test]
    fn serde_ner_test() {
        let json = "{\"text\":\"我爱北京天安门\",\"entity_analysis\":[{\"mention\":\"我爱北京天安门\",\"category\":{\"level_1\":\"作品\",\"level_2\":\"音乐作品\",\"level_3\":\"歌曲\"},\"confidence\":0.0046176789328455925,\"desc\":\"1970年齐兵演唱的歌曲\",\"status\":\"NIL\"}],\"log_id\":1727416471850840488}";
        let _: NerRet = serde_json::from_str(json).unwrap();
    }
}
