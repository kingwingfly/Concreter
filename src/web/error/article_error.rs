use axum::response::IntoResponse;
use snafu::Snafu;

use crate::model::DbError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum ArticleError {
    #[snafu(display("Article not found"), context(false))]
    ArticleNotFound { source: DbError },
}

pub type ArticleResult<T> = Result<T, ArticleError>;

impl IntoResponse for ArticleError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ArticleError::ArticleNotFound { .. } => {
                (axum::http::StatusCode::NOT_FOUND, "Article not found").into_response()
            }
        }
    }
}
