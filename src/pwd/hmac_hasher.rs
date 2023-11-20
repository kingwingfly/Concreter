use hmac::{Hmac, Mac};
use sha2::Sha512;

use crate::utils::b64::b64u_encode;

use super::ContentToHash;
use super::PwdResult;

pub fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> PwdResult<String> {
    let ContentToHash { content, salt } = to_hash;

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();

    let result = b64u_encode(hmac_result.into_bytes());

    Ok(result)
}
