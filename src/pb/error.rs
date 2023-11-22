use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum RpcError {}

pub type RpcResult<T> = Result<T, RpcError>;
