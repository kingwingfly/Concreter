use axum::{http::StatusCode, response::IntoResponse};
use snafu::Snafu;
use tracing::debug;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum LoginError {
    #[snafu(display("User not found"))]
    UserNotFound { source: crate::model::DbError },
    #[snafu(display("Password not match"), context(false))]
    PasswordNotMatch { source: crate::pwd::PwdError },
    #[snafu(display("No pwd"))]
    NoPwd,
    #[snafu(display("Login off failed"), context(false))]
    LoginOffFailed { source: self::CookieError },
}

pub type LoginResult<T> = Result<T, LoginError>;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum CookieError {
    #[snafu(display("Add or Remove cookie failed"), context(false))]
    Cookie { source: crate::token::TokenError },
}

pub type CookieResult<T> = Result<T, CookieError>;

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
