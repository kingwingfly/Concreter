mod article;
mod error;

pub use error::*;
use regex::Regex;
use std::sync::OnceLock;

use crate::{
    ctx::Ctx,
    model::{ArticleNew, ModelManager, ToStore},
    pb::{nlp_client, sym_client, ConvertMdRequest, NerRequest},
};

pub trait Analyzer {
    async fn analyze(ctx: &Ctx, mm: &ModelManager, article: ArticleNew) -> AnalyzeResult<()> {
        let content = article.content.clone();
        let field = article.field.clone();
        let mut to_store = ToStore::new(article);
        Self::ner(&mut to_store, &content, &field).await?;
        Self::sym(&mut to_store, &content).await?;
        to_store.store(ctx, mm).await?;
        Ok(())
    }

    async fn ner(to_store: &mut ToStore, text: &str, field: &str) -> AnalyzeResult<()> {
        let mut nlp_client = nlp_client().await;
        let chunk_size = 496;
        for c in text.chars().collect::<Vec<_>>().chunks(chunk_size) {
            let req = NerRequest {
                text: c.iter().collect::<String>(),
                field: field.into(),
            };
            let resp = nlp_client.ner(req).await?.into_inner();
            // {"name": {"attr_name1": "attr1", "attr_name2": "attr2"}, ...}
            let ner_json: serde_json::Value = serde_json::from_str(&resp.ner_ret)?;
            for (entity, attris) in ner_json.as_object().unwrap() {
                to_store.add_entity(entity.clone(), attris.clone());
            }
        }
        Ok(())
    }

    async fn sym(to_store: &mut ToStore, text: &str) -> AnalyzeResult<()> {
        let mut client = sym_client().await;
        let formulas = extract_formulas(text);
        for md in formulas {
            let req = ConvertMdRequest { md: md.to_owned() };
            let resp = client.convert_md_formula(req).await?.into_inner();
            to_store.add_formula(md, resp.sym);
        }
        Ok(())
    }
}

fn extract_formulas(text: &str) -> Vec<String> {
    static RE: OnceLock<Regex> = OnceLock::new();

    let ret: Vec<String> = RE
        .get_or_init(|| Regex::new(r"\$\$?(?P<formula>[\s\S]+?)\$\$?").expect("Invalid regex"))
        .captures_iter(text)
        .filter_map(|cap| cap.name("formula").map(|f| f.as_str().trim().to_string()))
        .filter(|s| s.contains("="))
        .collect();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_formulas_test() {
        let text = r#"
$z = x^2 + 2xy + y^2$
$z = h (h - 0.5x)$
$$
E = mc^2
$$
        "#;
        let formulas = extract_formulas(text);
        assert_eq!(formulas.len(), 3);
        assert_eq!(formulas[0], "z = x^2 + 2xy + y^2");
        assert_eq!(formulas[1], "z = h (h - 0.5x)");
        assert_eq!(formulas[2], "E = mc^2");
        let text = std::fs::read_to_string("examples/建筑智能施工技术.md").unwrap();
        let ret = extract_formulas(&text);
        dbg!(ret);
    }
}
