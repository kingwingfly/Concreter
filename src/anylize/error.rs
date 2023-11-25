use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum AnalyzeError {
    #[snafu(display("Failed RPC"), context(false))]
    FailedRpc { source: tonic::Status },
    #[snafu(display("Bad reply format"), context(false))]
    BadReplyFormat { source: serde_json::Error },
    #[snafu(display("Failed to store"), context(false))]
    FailedToStore { source: crate::model::DbError },
}

pub type AnalyzeResult<T> = Result<T, AnalyzeError>;
