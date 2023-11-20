use crate::ctx::Ctx;
use crate::model::{ModelManager, PgdbBmc, UserPg, UserPgBmc};
use crate::token::{validate_web_token, Token};
use crate::web::{set_token_cookie, AUTH_TOKEN};
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use snafu::OptionExt;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::{auth_error, AuthError, AuthResult};

pub async fn mw_ctx_require<B>(
    ctx: AuthResult<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> AuthResult<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> AuthResult<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(AuthError::TokenNotInCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> AuthResult<Ctx> {
    // -- Get Token String
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .context(auth_error::TokenNotInCookie)?;

    // -- Parse Token
    let token: Token = token
        .parse()
        .map_err(|_| auth_error::TokenWrongFormat.build())?;

    // -- Get UserForAuth
    let user: UserPg = UserPgBmc::first_by(&Ctx::root_ctx(), &mm, "username", &token.ident)
        .await
        .map_err(|e| auth_error::UserNotFound.build())?;

    // -- Validate Token
    validate_web_token(&token, user.token_salt).map_err(|_| auth_error::FailValidate.build())?;

    // -- Update Token
    set_token_cookie(cookies, &user.username, user.token_salt)
        .map_err(|_| auth_error::FailSetTokenCookie.build())?;

    // -- Create CtxExtResult
    Ctx::new(user.id).map_err(|_| auth_error::CtxCreateFail.build())
}

// region:    --- Ctx Extractor
#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("{:<12} - Ctx Extractor", "EXTRACTOR");

        parts
            .extensions
            .get::<AuthResult<Ctx>>()
            .context(auth_error::CtxNotInRequest)?
            .clone()
    }
}
// endregion: --- Ctx Extractor
