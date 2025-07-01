use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::{Ok, Result};
use dirs::home_dir;

mod preprocess;
mod sqlite_interface;
mod tf_idf;

use preprocess::preprocess;
use tf_idf::{
  tf_idf,
  all_document_tf_idf,
  every_term_tf_idf,
};

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
  println!("After: {:?}", processed.clone());

  let tf_idf_0 = tf_idf("cat".to_string(), processed[0].clone(), processed.clone());
  let tf_idf_1 = tf_idf("cat".to_string(), processed[1].clone(), processed.clone());
  println!("Cat in Doc 0: {tf_idf_0}");
  println!("Cat in Doc 1: {tf_idf_1}");

  let all_tf_idf = all_document_tf_idf("cat".to_string(), processed.clone());
  println!("Cat in all: {:?}", all_tf_idf);

  let every_term_0 = every_term_tf_idf(processed[0].clone(), processed.clone());
  let every_term_1 = every_term_tf_idf(processed[1].clone(), processed.clone());
  println!("Every term in 0: {:?}", every_term_0);
  println!("Every term in 1: {:?}", every_term_1);
  Ok(())
}
