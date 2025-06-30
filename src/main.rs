use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::{Ok, Result};
use dirs::home_dir;

mod preprocess;
mod sqlite_interface;

use preprocess::preprocess;

const PATH: &str = "dev/rust/page_compiler/src/data.db";

#[async_std::main]
async fn main() -> Result<()>{
  let path = home_dir()
    .expect("Unable to find home directory")
    .join(PATH)
    .into_os_string()
    .into_string()
    .unwrap();

  if !Sqlite::database_exists(&path).await.unwrap_or(false) {
    println!("Creating database: {}", &path);
    Sqlite::create_database(&path).await?;
  }

  let db = SqlitePool::connect(&path).await.unwrap();

  sqlite_interface::init(&db).await?;

  let data = vec!("Cats chase mice, dogs bark loudly, and birds fly south in winter.".to_string(), "Running quickly exhausted him, but he kept jogged until he collapsed on the grass.".to_string());
  println!("Before: {:?}", data.clone());
  let processed = preprocess(data);
  println!("After: {:?}", processed);

  Ok(())
}
