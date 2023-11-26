use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use snafu::ResultExt;
use tower_cookies::Cookies;
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{ModelManager, PgdbBmc, UserPg, UserPgBmc, UserPgNew},
    pwd::{self, hash_pwd, ContentToHash},
    web::{self, login_error, remove_token_cookie},
};

use super::LoginResult;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .route("/api/register", post(api_register_handler))
        .with_state(mm)
}

// region:    --- Login

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
    // let user_id = user.id;

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
    #[serde(alias = "password")]
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

// region:    --- Register

async fn api_register_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<RegisterPayload>,
) -> LoginResult<Json<Value>> {
    let RegisterPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let ctx = Ctx::root_ctx();
    let user_pg = UserPgNew { username };
    let id = UserPgBmc::insert(&ctx, &mm, user_pg).await?;
    let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "id", id).await?;
    let pwd = hash_pwd(&ContentToHash {
        content: pwd_clear,
        salt: user.pwd_salt,
    })?;
    UserPgBmc::update_one_field(&ctx, &mm, &user, "pwd", pwd).await?;
    web::set_token_cookie(&cookies, &user.username, user.token_salt)?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    pwd: String,
}
// endregion: --- Register
