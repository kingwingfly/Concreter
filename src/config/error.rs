use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum ConfigError {
    #[snafu(display("missing env {}", env_virable))]
    MissingEnv {
        source: std::env::VarError,
        env_virable: String,
    },
    #[snafu(display("wrong format"))]
    WrongFormat,
}

pub type ConfigResult<T> = Result<T, ConfigError>;
