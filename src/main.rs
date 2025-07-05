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

  let input = "lua is a great scripting language that can be used with other programming languages like rust, it is similar in simplicty to python, however lua is more so used in embedded applications and in some cases game development".to_string();
  let data = vec![
    "machine learning is fun".to_string(),                     // Doc 0
    "deep learning is powerful".to_string(),                   // Doc 1
    "artificial intelligence is the future".to_string(),       // Doc 2
    "machine intelligence is rising".to_string(),              // Doc 3
    "rust programming is fast and safe".to_string(),           // Doc 4
    "python is great for data science".to_string(),            // Doc 5
    "data science requires statistics".to_string(),            // Doc 6
    "statistics is the backbone of ML".to_string(),            // Doc 7
    "deep neural networks are revolutionary".to_string(),      // Doc 8
    "rust and python are both awesome languages".to_string(),  // Doc 9
  ];

  let stop_words = get(LANGUAGE::English);

  let tfidf_data = corpus_tfidf_preprocess(data.clone(), stop_words.clone());
  let rake_data = corpus_rake_preprocess(data.clone(), stop_words.clone());
  let tfidf_input = tfidf_preprocess(input.clone(), stop_words.clone());
  let rake_input = rake_preprocess(input.clone(), stop_words.clone());

  let scores = similarity::combined_similarity_scores(tfidf_input, rake_input, tfidf_data, rake_data, COSINE_WEIGHT);

  for score in scores.clone() {
    println!("{} combined_scores to input: {}", score.0, score.1);
  }

  println!("{} has the highest score of {}", scores[0].0, scores[0].1);

  Ok(())
}
