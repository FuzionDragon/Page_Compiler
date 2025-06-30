use async_std::prelude::*;
mod preprocess;
mod sqlite_interface;

use anyhow::{Ok, Result};
use preprocess::preprocess;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const PATH: &str = "data.db";

#[async_std::main]
async fn main() -> Result<()>{
  println!("Hello, world!");

  if !Sqlite::database_exists(PATH).await.unwrap_or(false) {
    println!("Creating database: {}", PATH);
    Sqlite::create_database(PATH).await?;
  }

  let db = SqlitePool::connect(PATH).await.unwrap();

  sqlite_interface::init(&db).await?;

  Ok(())
}
