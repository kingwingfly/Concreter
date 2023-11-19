use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use snafu::ResultExt;
use tower_cookies::Cookies;
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{ModelManager, PgdbBmc, UserPg, UserPgBmc},
    pwd::{self, ContentToHash},
    web::{self, login_error, remove_token_cookie},
};

use super::LoginResult;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .with_state(mm)
}

// region:    --- Login

#[axum::debug_handler]
async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> LoginResult<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user.
    let user: UserPg = UserPgBmc::first_by(&root_ctx, &mm, "username", &username)
        .await
        .context(login_error::UserNotFound)?;
    let user_id = user.id;

    // -- Validate the password.
    let Some(pwd) = user.pwd else {
        return login_error::NoPwd.fail();
    };

    pwd::validate_pwd(
        &ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        &pwd,
    )?;

    // -- Set web token.
    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
// endregion: --- Login

// region:    --- Logoff
async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> LoginResult<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    // Create the success body.
    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}
// endregion: --- Logoff
