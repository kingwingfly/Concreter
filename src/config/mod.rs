// region:    --- Modules

mod error;

pub use error::*;
use snafu::ResultExt;

use crate::utils::b64::b64u_decode;
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

// endregion: --- Modules

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Keys
    pub PWD_KEY: Vec<u8>,

    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: f64,

    // -- Db
    pub PG_URL: String,
    pub AG_FILE: String,

    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> ConfigResult<Self> {
        Ok(Self {
            // -- Keys
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,

            // -- Db
            PG_URL: get_env("PG_URL")?,
            AG_FILE: get_env("AG_FILE")?,

            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> ConfigResult<String> {
    Ok(env::var(name).context(config_error::MissingEnv {
        env_virable: name.to_owned(),
    })?)
}

fn get_env_parse<F: FromStr>(name: &'static str) -> ConfigResult<F> {
    let val = get_env(name)?;
    Ok(val
        .parse::<F>()
        .map_err(|_| config_error::WrongFormat.build())?)
}

fn get_env_b64u_as_u8s(name: &'static str) -> ConfigResult<Vec<u8>> {
    Ok(b64u_decode(&get_env(name)?).map_err(|_| config_error::WrongFormat.build())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let _ = config();
    }
}
