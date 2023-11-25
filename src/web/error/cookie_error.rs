use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum CookieError {
    #[snafu(display("Add or Remove cookie failed"), context(false))]
    Cookie { source: crate::token::TokenError },
}

pub type CookieResult<T> = Result<T, CookieError>;
