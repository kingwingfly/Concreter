use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module, visibility(pub), context(suffix(Error)))]
pub enum DbError {
    #[snafu(display("Create agdb failed"), context(false))]
    AgDbCreate { source: agdb::DbError },
    #[snafu(display("Exec agdb failed"), context(false))]
    DbExec { source: agdb::QueryError },
    #[snafu(display("Sqlx failed"), context(false))]
    Sqlx { source: sqlx::Error },
}

pub type DbResult<T> = Result<T, DbError>;
