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
    fn article_analyze_test() {
        run_test(async {
            let ctx = Ctx::root_ctx();
            let mm = init_test().await;
            let article = ArticleNew {
                author: 1000,
                title: "建筑智能施工技术".into(),
                content:
                // std::fs::read_to_string("examples/建筑智能施工技术.md").unwrap()
                "紧张刺激的飞行棋游戏，让你的飞机飞行到终点，成为胜利者。".to_string()
                ,
                field: "游戏".into(),
            };
            ArticleAnalyzer::analyze(&ctx, &mm, article).await.unwrap();
        })
    }
}
