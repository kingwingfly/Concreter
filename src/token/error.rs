use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum TokenError {
    #[snafu(display("InvalidFormat"))]
    InvalidFormat,
    #[snafu(display("CannotDecodeIdent"))]
    CannotDecodeIdent { source: crate::utils::b64::B64Error },
    #[snafu(display("CannotDecodeExp"))]
    CannotDecodeExp { source: crate::utils::b64::B64Error },
    #[snafu(display("ExpNotIso"), context(false))]
    ExpNotIso {
        source: crate::utils::time::TimeError,
    },
    #[snafu(display("HmacFailNewFromSlice"), context(false))]
    HmacFailNewFromSlice { source: hmac::digest::InvalidLength },
    #[snafu(display("SignatureNotMatching"))]
    SignatureNotMatching,
    #[snafu(display("TokenExpired"))]
    Expired,
}

pub type TokenResult<T> = Result<T, TokenError>;
