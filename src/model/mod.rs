mod base;
mod macros;

mod article;
mod comment;
mod formula;
mod user;
mod word;

mod error;

use agdb::{Db, QueryBuilder, QueryError};

pub use error::*;

pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub fn new() -> DbResult<Self> {
        #[cfg(test)]
        dotenv::dotenv().ok();

        let filename = std::env::var("AgdbFile")?;
        let mut db = Db::new(&filename)?;

        init_agdb(&mut db)?;

        Ok(Self { db })
    }

    fn agdb(&self) -> &Db {
        &self.db
    }

    fn agdb_mut(&mut self) -> &mut Db {
        &mut self.db
    }
}

fn init_agdb(db: &mut Db) -> DbResult<()> {
    Ok(db.transaction_mut(|t| -> Result<(), QueryError> {
        for name in vec!["root", "articles", "users", "words", "formulas", "comments"] {
            match t.exec_mut(&QueryBuilder::insert().nodes().aliases(name).query()) {
                Ok(_) => {
                    t.exec_mut(&QueryBuilder::insert().edges().from("root").to(name).query())?;
                }
                Err(e) if e.description.contains("exists") => continue,
                Err(e) => Err(e)?,
            };
        }
        Ok(())
    })?)
}

#[cfg(test)]
mod create_mm_test {
    use super::*;

    #[test]
    fn create_mm_test() {
        let _mm = ModelManager::new().unwrap();
    }
}
