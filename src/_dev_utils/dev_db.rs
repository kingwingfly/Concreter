use std::{fs, path::PathBuf};

use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgConnection};
use tracing::info;
use url::Url;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:postgres@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

const SQL_CREATEDB_FILE_NAME: &str = "00-createdb.sql";
const SQL_DIR: &str = "sql/dev_init";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    let sql_dir = std::env::current_dir()?.join(SQL_DIR);
    {
        let mut con = PgConnectOptions::from_url(&Url::parse(PG_DEV_POSTGRES_URL)?)?
            .connect()
            .await?;
        pexe(&mut con, sql_dir.join(SQL_CREATEDB_FILE_NAME)).await?;
    }
    let mut con = PgConnectOptions::from_url(&Url::parse(PG_DEV_APP_URL)?)?
        .connect()
        .await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();
    for path in paths {
        let path_str = path.to_string_lossy();
        if path_str.ends_with(".sql") && !path_str.ends_with(SQL_CREATEDB_FILE_NAME) {
            pexe(&mut con, path).await?;
        }
    }

    Ok(())
}

async fn pexe(con: &mut PgConnection, filepath: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    info!("Executing {}", filepath.to_string_lossy());
    let sqls = std::fs::read_to_string(filepath)?;
    for sql in sqls.split(';') {
        sqlx::query(sql.trim()).execute(&mut *con).await?;
    }

    Ok(())
}
