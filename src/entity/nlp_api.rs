//! Use Api at https://www.nlp-api.com/documentation.
//! However, it was almost unusable when processing Chinese text.

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};

use crate::config::config;

use super::NerResult;

pub struct Ner {
    client: Client,
}

#[derive(serde::Serialize)]
struct Request {
    api_token: String,
    text: String,
    language: String,
}

impl Request {
    fn new(text: String) -> Self {
        Self {
            api_token: config().NLP_API_TOKEN.to_owned(),
            text,
            language: "zh".to_owned(),
        }
    }
}

impl Ner {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        Self {
            client: Client::builder().default_headers(headers).build().unwrap(),
        }
    }

    pub async fn ner(&self, text: String) -> NerResult<NerRet> {
        let url = "https://api.nlp-api.com/v1/ner"
            .parse::<url::Url>()
            .unwrap();
        let resp = self
            .client
            .post(url)
            .json(&Request::new(text))
            .send()
            .await
            .unwrap();
        let ret: NerRet = resp.json().await.unwrap();
        Ok(ret)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct NerRet {
    data: Vec<Entity>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Entity {
    text: String,
    start_char: usize,
    end_char: usize,
    label: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Read};

    #[tokio::test]
    async fn nlp_api_ner_test() {
        let ner = Ner::new();
        let mut f = fs::File::open("examples/建筑工程合同管理.md").unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        let chunk_size = 496;
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
}
