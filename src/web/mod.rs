mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_article;
pub mod routes_login;
pub mod routes_static;
pub mod rpc;

pub use error::*;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::token::generate_web_token;

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> CookieResult<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> CookieResult<()> {
    let mut cookie = Cookie::named(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
