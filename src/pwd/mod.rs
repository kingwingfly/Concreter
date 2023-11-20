mod error;
mod hmac_hasher;

pub use error::*;
use uuid::Uuid;

use crate::config::config;
use hmac_hasher::hmac_sha512_hash;

pub struct ContentToHash {
    pub content: String,
    pub salt: Uuid,
}

pub fn hash_pwd(to_hash: &ContentToHash) -> PwdResult<String> {
    let key = &config().PWD_KEY;

    let hashed = hmac_sha512_hash(key, to_hash)?;

    Ok(format!("#01#{hashed}"))
}

pub fn validate_pwd(enc_content: &ContentToHash, pwd_ref: &str) -> PwdResult<()> {
    let pwd = hash_pwd(enc_content)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        pwd_error::PwdNotMatch.fail()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        _dev_utils::{init_test, run_test},
        ctx::Ctx,
        model::{PgdbBmc, UserPg, UserPgBmc},
    };

    use super::*;

    #[test]
    fn pwd_test() {
        run_test(async {
            let ctx = Ctx::root_ctx();
            let mm = init_test().await;
            let pwd = "welcome";
            let user: UserPg = UserPgBmc::first_by(&ctx, &mm, "username", "demo1")
                .await
                .unwrap();
            let pwd_salt = user.pwd_salt;
            let content_to_hash = ContentToHash {
                content: pwd.to_string(),
                salt: pwd_salt,
            };
            let pwd_hashed = hash_pwd(&content_to_hash).unwrap();
            assert!(validate_pwd(&content_to_hash, &pwd_hashed).is_ok())
        })
    }
}
