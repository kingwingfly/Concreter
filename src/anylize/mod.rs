mod article;
mod error;

pub use error::*;

use crate::{
    model::ToStore,
    pb::{nlp_client, NerRequest},
};

pub trait Analyzer {
    async fn analyze(text: &str) -> AnalyzeResult<()> {
        Ok(())
    }

    async fn ner(to_store: &mut ToStore, text: &str, region: &str) -> AnalyzeResult<()> {
        let mut nlp_client = nlp_client().await;
        let req = NerRequest {
            text: text.into(),
            region: region.into(),
        };
        let resp = nlp_client.ner(req).await?.into_inner();
        // {"name": {"attr_name1": "attr1", "attr_name2": "attr2"}, ...}
        let ner_json: serde_json::Value = serde_json::from_str(&resp.ner_ret)?;
        for (entity, attris) in ner_json.as_object().unwrap() {
            to_store.add_entity(entity.clone(), attris.clone());
        }
        Ok(())
    }
}
