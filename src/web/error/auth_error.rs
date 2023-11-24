use axum::{http::StatusCode, response::IntoResponse};
use snafu::Snafu;
use tracing::debug;

#[derive(Debug, Snafu, Clone)]
#[snafu(visibility(pub), context(suffix(false)))]
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
