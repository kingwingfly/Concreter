use axum::response::IntoResponse;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum FormulaError {
    #[snafu(display("Formula not found"), context(false))]
    FormulaNotFound { source: crate::model::DbError },
}

pub type FormulaResult<T> = Result<T, FormulaError>;

impl IntoResponse for FormulaError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FormulaError::FormulaNotFound { .. } => {
                (axum::http::StatusCode::NOT_FOUND, "Formula not found").into_response()
            }
        }
    }
}
