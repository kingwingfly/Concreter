use std::{fs, path::PathBuf};

use serde_json::json;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgConnection, Row};
use tracing::info;
use url::Url;

use crate::{
    ctx::Ctx,
    model::{ArticleNew, ModelManager, ToStore},
    pwd::{hash_pwd, ContentToHash},
};

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost:5432/app_db";

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

    init_user_demo1(con).await?;
    init_entity().await?;

    Ok(())
}

async fn init_user_demo1(mut con: PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    let pwd_salt = sqlx::query("SELECT pwd_salt FROM users where username=$1 LIMIT 1")
        .bind("demo1")
        .fetch_one(&mut con)
        .await
        .unwrap()
        .try_get(0)
        .unwrap();
    let pwd = "welcome";
    let content_to_hash = ContentToHash {
        content: pwd.to_string(),
        salt: pwd_salt,
    };
    let pwd_hashed = hash_pwd(&content_to_hash).unwrap();
    sqlx::query("UPDATE users SET pwd=$1 WHERE username='demo1'")
        .bind(pwd_hashed)
        .execute(&mut con)
        .await?;
    Ok(())
}

async fn init_entity() -> Result<(), Box<dyn std::error::Error>> {
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();
    let to_store = ToStore::new(
        &ctx,
        &mm,
        ArticleNew {
            author: 1000,
            title: "hello".to_owned(),
            content: "world".to_owned(),
            field: "".to_owned(),
        },
    )
    .await
    .unwrap();
    let json = json!({
        "name": {
            "attr_name1": "attr1",
            "attr_name2": {
                "sub_attr_name1": "sub_attr1",
                "sub_attr_name2": "sub_attr2"
            }
        }
    });
    for (name, attris) in json.as_object().unwrap() {
        to_store
            .add_entity(&ctx, &mm, name.to_owned(), attris.to_owned())
            .await
            .unwrap();
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
