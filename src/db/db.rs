use std::fs;
use sqlx::{Error, SqlitePool};

#[derive(Debug)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        //TODO 여러 DB 연결가능한 구조로 변경
        let pool = SqlitePool::connect(database_url).await?;
        let database = Database { pool };

        database.init_tables().await?;

        Ok(database)
    }

    async fn init_tables(&self) -> Result<(), Error>{
        let sql = fs::read_to_string("src/sql/create_tables.sql")
            .expect("Failed to read SQL file");

        sqlx::query(&sql)
            .execute(&self.pool)
            .await?;

        Ok(())
    }


    /*
    DB Pull을 얻음
     */
    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
