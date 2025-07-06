use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use stop_words::{ get, LANGUAGE };
use anyhow::{Ok, Result};
use dirs::home_dir;

mod sqlite_interface;
mod similarity;
mod preprocess;
mod tf_idf;
mod rake;

use preprocess::*;

const PATH: &str = "dev/rust/page_compiler/src/data.db";
const COSINE_WEIGHT: f32 = 0.4;
const THESHOLD: f32 = 0.6;

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

  let input = "lua is a great scripting language that can be used with other programming languages like rust, it is similar in simplicty to python, however lua is more so used in embedded applications and in some cases game development";

  let data = vec![
    "machine learning is fun",                     // Doc 0
    "deep learning is powerful",                   // Doc 1
    "artificial intelligence is the future",       // Doc 2
    "machine intelligence is rising",              // Doc 3
    "rust programming is fast and safe",           // Doc 4
    "python is great for data science",            // Doc 5
    "data science requires statistics",            // Doc 6
    "statistics is the backbone of ML",            // Doc 7
    "deep neural networks are revolutionary",      // Doc 8
    "rust and python are both awesome languages",  // Doc 9
  ];

  let stop_words = get(LANGUAGE::English);

  let corpus_tfidf_data = corpus_tfidf_preprocess(data.clone(), stop_words.clone());
  let corpus_rake_data = corpus_rake_preprocess(data, stop_words.clone());
  let input_tfidf_data = tfidf_preprocess(input, stop_words.clone());
  let input_rake_data = rake_preprocess(input, stop_words.clone());

  let scores = similarity::combined_similarity_scores(input_tfidf_data, input_rake_data, corpus_tfidf_data, corpus_rake_data, COSINE_WEIGHT);

  for score in scores.clone() {
    println!("{} combined_scores to input: {}", score.0, score.1);
  }

  println!("{} has the highest score of {}", scores[0].0, scores[0].1);

  Ok(())
}
