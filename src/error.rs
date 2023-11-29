use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(false)))]
pub enum AppError {
    #[snafu(display("ModelManager Error"), context(false))]
    ModelManager { source: crate::model::DbError },
    #[snafu(display("InitFaild"))]
    InitFaild,
}

pub type AppResult<T> = Result<T, AppError>;
