use std::collections::HashMap;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::{Ok, Result};
use dirs::home_dir;

mod sqlite_interface;
mod similarity;
mod preprocess;
mod tf_idf;
mod rake;

use preprocess::*;
use tf_idf::{ all_tf_idf_vectorize, compute_all_idf, tf_idf, tf_idf_vectorize };
use rake::{ all_rake, rake };

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

  let input = "lua is a great scripting language that can be used with other programming languages like rust".to_string();
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

  let tfidf_data = corpus_tfidf_preprocess(data.clone());
  let rake_data = corpus_rake_preprocess(data.clone());

  let all_tfidf_scores = all_tf_idf_vectorize(tfidf_data.clone());
  let all_rake_scores = all_rake(rake_data.clone());

  let rake_input = rake_preprocess(input.clone());
  let tf_idf_input = rake_preprocess(input);
  let input_top_terms = rake(rake_input.clone());
  let tf_idf_input_score = tf_idf_vectorize(tf_idf_input, tfidf_data);

  for scores in all_tfidf_scores {
    println!("{} cosine similarity to input: {:?}", scores.0, similarity::cosine_similarity(tf_idf_input_score.clone(), scores.1));
  }

  for scores in all_rake_scores.clone() {
    println!("{} regular jaccard similarity to input: {:?}", scores.0, similarity::jaccards_similarity(rake_input.clone(), rake_data[scores.0].clone()));
  }

  for scores in all_rake_scores.clone() {
    println!("{} weighted jaccard similarity to input: {:?}", scores.0, similarity::weighted_jaccard_similarity(rake_input.clone(), rake_data[scores.0].clone(), input_top_terms.clone(), all_rake_scores[&scores.0].clone()));
  }

  Ok(())
}

fn combine_results(tf_idf_scores: HashMap<String, f32>, rake_scores: Vec<(f32, String)>) {

}
