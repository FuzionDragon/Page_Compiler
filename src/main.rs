use std::collections::HashMap;

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

pub type CorpusSnippets = HashMap<String, Vec<String>>;
pub type StrCorpusSnippets <'a> = HashMap<String, Vec<&'a str>>;
pub type Corpus = HashMap<String, String>;

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

  let data = get_test_corpus();

  let stop_words = get(LANGUAGE::English);

  // make sure to retrieve scores for tfidf and rake from previous processing, unless this is the
  // first run on the notes.
  // in the future this is only done after the checking is done and the snippet is in a new
  // document or created a new one

  // job for next time, make sure the corpus preprocessing and processing return hashmaps with the
  // document names instead of usize, 
  //
  // PS screw you past me
  let corpus_tfidf_data = corpus_tfidf_preprocess(data.clone(), stop_words.clone());
  let corpus_rake_data = corpus_rake_preprocess(data, stop_words.clone());
  let input_tfidf_data = tfidf_preprocess(input, stop_words.clone());
  let input_rake_data = rake_preprocess(input, stop_words.clone());
  println!("{:?}", corpus_tfidf_data.clone());
  println!();
  println!("{:?}", corpus_rake_data.clone());
  println!();
  println!("{:?}", input_tfidf_data.clone());
  println!();
  println!("{:?}", input_rake_data.clone());

  let scores = similarity::combined_similarity_scores(input_tfidf_data, input_rake_data, corpus_tfidf_data, corpus_rake_data, COSINE_WEIGHT);

  for score in scores.clone() {
    println!("{} combined_scores to input: {}", score.0, score.1);
  }

  if scores[0].1 >= THESHOLD {
    println!("{} is the chosen document with a score of {}", scores[0].0, scores[0].1);
  } else {
    println!("{} doesn't meet the threshold with a score of {}", scores[0].0, scores[0].1);
  }

  Ok(())
}

fn get_test_corpus() -> HashMap<String, String> {
  let mut corpus = HashMap::new();

  corpus.insert(
    "doc1".to_string(),
    "machine learning is fun".to_string(),
  );

  corpus.insert(
    "doc2".to_string(),
    "deep learning is powerful".to_string(),
  );

  corpus.insert(
    "doc3".to_string(),
    "artificial intelligence is the future".to_string(),
  );

  corpus.insert(
    "doc4".to_string(),
    "machine intelligence is rising".to_string(),
  );

  corpus.insert(
    "doc5".to_string(),
    "rust programming is fast and safe".to_string(),
  );

  corpus.insert(
    "doc6".to_string(),
    "python is great for data science".to_string(),
  );

  corpus.insert(
    "doc7".to_string(),
    "data science requires statistics".to_string(),
  );

  corpus.insert(
    "doc8".to_string(),
    "statistics is the backbone of ML".to_string(),
  );

  corpus.insert(
    "doc9".to_string(),
    "deep neural networks are revolutionary".to_string(),
  );

  corpus.insert(
    "doc10".to_string(),
    "rust and python are both awesome languages".to_string(),
  );

  corpus
}
