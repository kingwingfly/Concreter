use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(Error)))]
pub enum NerError {
    #[snafu(display("failed to get access token of baidu ner"))]
    GetAccess,
    #[snafu(display("failed to ner sentence: {}", text), context(suffix(false)))]
    FaildNer {
        source: reqwest::Error,
        text: String,
    },
    #[snafu(display("failed to parse json"))]
    DecodeError,
}

pub type NerResult<T> = Result<T, NerError>;
