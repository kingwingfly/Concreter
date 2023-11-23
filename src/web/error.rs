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
    #[snafu(display("Register failed"), context(false))]
    RegisterFailed { source: crate::model::DbError },
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
        let mut response = match self {
            LoginError::UserNotFound { .. } => {
                (StatusCode::UNAUTHORIZED, "User not found").into_response()
            }
            LoginError::PasswordNotMatch { .. } => {
                (StatusCode::UNAUTHORIZED, "Password not match").into_response()
            }
            LoginError::NoPwd => {
                (StatusCode::UNAUTHORIZED, "No password on server stored").into_response()
            }
            LoginError::LoginOffFailed { .. } => {
                (StatusCode::BAD_REQUEST, "Login off failed").into_response()
            }
            LoginError::RegisterFailed { .. } => {
                (StatusCode::BAD_REQUEST, "Register failed").into_response()
            }
        };

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - web::AuthError {self:?}", "INTO_RES");
        // todo match errors
        // Create a placeholder Axum reponse.
        let mut response = match self {
            AuthError::TokenNotInCookie
            | AuthError::TokenWrongFormat
            | AuthError::FailValidate
            | AuthError::UserNotFound => (StatusCode::UNAUTHORIZED).into_response(),
            AuthError::FailSetTokenCookie
            | AuthError::CtxCreateFail
            | AuthError::CtxNotInRequest => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
