use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(Error)))]
pub enum DbError {
    #[snafu(display("DbFile env var not found"), context(false))]
    EnvError { source: std::env::VarError },
    #[snafu(display("Create agdb failed"), context(false))]
    DbCreateError { source: agdb::DbError },
    #[snafu(display("Exec agdb failed"), context(false))]
    DbExecError { source: agdb::QueryError },
}

pub type DbResult<T> = Result<T, DbError>;
