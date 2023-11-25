use super::Analyzer;

pub struct ArticleAnalyzer {}

impl Analyzer for ArticleAnalyzer {}

#[cfg(test)]
mod test {
    use crate::{
        _dev_utils::{init_test, run_test},
        ctx::Ctx,
        model::ArticleNew,
    };

    use super::*;

    #[test]
    fn article_anylyzer_test() {
        run_test(async {
            let ctx = Ctx::root_ctx();
            let mm = init_test().await;
            let article = ArticleNew {
                author: 1000,
                title: "建筑智能施工技术".into(),
                content: std::fs::read_to_string("examples/建筑智能施工技术.md").unwrap(),
                field: "BIM 建筑 施工技术".into(),
            };
            ArticleAnalyzer::analyze(&ctx, &mm, article).await.unwrap();
        })
    }
}
