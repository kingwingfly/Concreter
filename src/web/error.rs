use axum::{http::StatusCode, response::IntoResponse};
use snafu::Snafu;
use tracing::debug;

use crate::model::DbError;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum LoginError {
    #[snafu(display("User not found"))]
    UserNotFound { source: DbError },
    #[snafu(display("Password not match"), context(false))]
    PasswordNotMatch { source: crate::pwd::PwdError },
    #[snafu(display("No pwd"))]
    NoPwd,
    #[snafu(display("Login off failed"), context(false))]
    LoginOffFailed { source: CookieError },
}

pub type LoginResult<T> = Result<T, LoginError>;

#[derive(Debug, Snafu, Clone)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum AuthError {
    #[snafu(display("TokenNotInCookie"))]
    TokenNotInCookie,
    #[snafu(display("TokenWrongFormat"))]
    TokenWrongFormat,
    #[snafu(display("UserNotFound"))]
    UserNotFound,
    #[snafu(display("AuthFailValidate"))]
    FailValidate,
    #[snafu(display("CannotSetTokenCookie"))]
    FailSetTokenCookie,
    #[snafu(display("CtxCreateFail"))]
    CtxCreateFail,
    #[snafu(display("CtxNotInRequest"))]
    CtxNotInRequest,
}

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum CookieError {
    #[snafu(display("Add or Remove cookie failed"), context(false))]
    Cookie { source: crate::token::TokenError },
}

pub type CookieResult<T> = Result<T, CookieError>;

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - web::LoginError {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - web::AuthError {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
