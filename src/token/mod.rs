mod error;

use std::{fmt::Display, str::FromStr};

pub use error::*;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use snafu::ResultExt;
use uuid::Uuid;

use crate::{
    config::config,
    utils::{
        b64::{b64u_decode_to_string, b64u_encode},
        time::{now_utc, now_utc_plus_sec_str, parse_utc},
    },
};

// endregion: --- Modules

// region:    --- Token Type

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    pub ident: String,     // Identifier (username for example).
    pub exp: String,       // Expiration date in Rfc3339.
    pub sign_b64u: String, // Signature, base64url encoded.
}

impl FromStr for Token {
    type Err = TokenError;

    fn from_str(token_str: &str) -> TokenResult<Self> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return token_error::InvalidFormat.fail();
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode_to_string(ident_b64u).context(token_error::CannotDecodeIdent)?,
            exp: b64u_decode_to_string(exp_b64u).context(token_error::CannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation

pub fn generate_web_token(user: &str, salt: Uuid) -> TokenResult<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> TokenResult<()> {
    let config = &config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Token Gen and Validation

fn _generate_token(ident: &str, duration_sec: f64, salt: Uuid, key: &[u8]) -> TokenResult<Token> {
    // -- Compute the two first components.
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // -- Sign the two first components.
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: Uuid, key: &[u8]) -> TokenResult<()> {
    // -- Validate signature.
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    if new_sign_b64u != origin_token.sign_b64u {
        return token_error::SignatureNotMatching.fail();
    }

    // -- Validate expiration.
    let origin_exp = parse_utc(&origin_token.exp)?;
    let now = now_utc();

    if origin_exp < now {
        return token_error::Expired.fail();
    }

    Ok(())
}

/// Create token signature from token parts
/// and salt.
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: Uuid, key: &[u8]) -> TokenResult<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}

// endregion: --- (private) Token Gen and Validation
