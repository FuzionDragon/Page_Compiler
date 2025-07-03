use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::{Ok, Result};
use dirs::home_dir;

mod preprocess;
mod sqlite_interface;
mod tf_idf;
mod rake;

use preprocess::{ tfidf_preprocess, rake_preprocess, all_terms };
use tf_idf::{ tf_idf_vectorize, tf_idf, all_tf_idf_vectorize };

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

  let data = vec!("Cats chase mice, dogs bark loudly, and birds fly south in winter.".to_string(), "Running cat quickly exhausted him, but he kept jogged until he collapsed on the grass.".to_string());

  let tfidf_data = tfidf_preprocess(data.clone());
  let rake_data = rake_preprocess(data.clone());

  println!("{:?}", tfidf_data.clone());
  let all_tfidf_terms = all_terms(tfidf_data.clone());
  println!("{:?}", all_tfidf_terms);

  let rake = rake::rake(rake_data[0].clone());
  println!("{:?}", rake);

  let tf_idf = tf_idf("cat".to_string(), tfidf_data[0].clone(), tfidf_data.clone());
  println!("{:?}", tf_idf);

  let doc_1_vec = tf_idf_vectorize(tfidf_data[0].clone(), tfidf_data.clone());
  let doc_2_vec = tf_idf_vectorize(tfidf_data[1].clone(), tfidf_data.clone());
  println!("Doc 1: {:?}", doc_1_vec);
  println!("Doc 2: {:?}", doc_2_vec);

  let all_vec = all_tf_idf_vectorize(tfidf_data);
  println!("All vectors: {:?}", all_vec);

  Ok(())
}
