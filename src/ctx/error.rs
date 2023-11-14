use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(module, visibility(pub), context(suffix(Error)))]
pub enum CtxError {
    #[snafu(display("illage user id 0"))]
    IllageUserId,
}

pub type CtxResult<T> = Result<T, CtxError>;
