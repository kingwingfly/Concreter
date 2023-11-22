mod error;

pub use error::*;
use snafu::ResultExt;

use crate::utils::b64::b64u_decode;
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

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

    // -- Baidu NER
    pub API_KEY: String,
    pub SECRET_KEY: String,

    // -- NLP_API
    pub NLP_API_TOKEN: String,

    // -- Db
    pub PG_URL: String,
    pub AG_FILE: String,

    // -- Web
    pub WEB_FOLDER: String,

    // --Rpc
    pub RPC_ADDR: String,
}

impl Config {
    fn load_from_env() -> ConfigResult<Self> {
        dotenv::dotenv().ok();
        Ok(Self {
            // -- Keys
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,

            // -- Baidu NER
            API_KEY: get_env("API_KEY")?,
            SECRET_KEY: get_env("SECRET_KEY")?,

            // -- NLP_API
            NLP_API_TOKEN: get_env("NLP_API_TOKEN")?,

            // -- Db
            PG_URL: get_env("PG_URL")?,
            AG_FILE: get_env("AG_FILE")?,

            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,

            // --Rpc
            RPC_ADDR: get_env("RPC_ADDR")?,
        })
    }
}

fn get_env(name: &'static str) -> ConfigResult<String> {
    env::var(name).context(config_error::MissingEnv {
        env_virable: name.to_owned(),
    })
}

fn get_env_parse<F: FromStr>(name: &'static str) -> ConfigResult<F> {
    let val = get_env(name)?;
    val.parse::<F>()
        .map_err(|_| config_error::WrongFormat.build())
}

fn get_env_b64u_as_u8s(name: &'static str) -> ConfigResult<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| config_error::WrongFormat.build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let _ = config();
    }
}
