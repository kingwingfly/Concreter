use base64::engine::{general_purpose, Engine};
use snafu::Snafu;

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> B64Result<Vec<u8>> {
    Ok(general_purpose::URL_SAFE_NO_PAD.decode(b64u)?)
}

pub fn b64u_decode_to_string(b64u: &str) -> B64Result<String> {
    b64u_decode(b64u).and_then(|r| Ok(String::from_utf8(r)?))
}

// region:    --- Error

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum B64Error {
    #[snafu(display("Fail to b64u decode"), context(false))]
    FailToB64uDecode { source: base64::DecodeError },
    #[snafu(display("Fail to b64u decode to String"), context(false))]
    NotUtf8 { source: std::string::FromUtf8Error },
}

pub type B64Result<T> = Result<T, B64Error>;
// endregion: --- Error Boilerplate

// endregion: --- Error
