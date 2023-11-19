use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(module, visibility(pub), context(suffix(Error)))]
pub enum PwdError {
    #[snafu(display("pwd not match"), context(suffix(false)))]
    PwdNotMatch,
    #[snafu(display("invalid key length"), context(false))]
    InvalidKeyLength { source: hmac::digest::InvalidLength },
}

pub type PwdResult<T> = Result<T, PwdError>;
