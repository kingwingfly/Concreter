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
