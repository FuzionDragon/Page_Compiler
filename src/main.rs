use std::collections::{ HashSet, HashMap };

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use stop_words::{ get, LANGUAGE };
use anyhow::{Ok, Result};
use dirs::home_dir;

mod sqlite_interface;
mod similarity;
mod preprocess;
mod tf_idf;
mod rake;

pub type CorpusSnippets = HashMap<String, Vec<String>>;
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

  let snippet = "lua is a great scripting language that can be used with other programming languages like rust, it is similar in simplicty to python, however lua is more so used in embedded applications and in some cases game development";

  submit_snippet(snippet, &db).await?;

  Ok(())
}

async fn submit_snippet(snippet: &str, db: &SqlitePool) -> Result<()> {
  let checker = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='Document'")
    .fetch_all(db)
    .await?
    .is_empty();

  let stop_words = get(LANGUAGE::English);

  let input_tfidf_data = preprocess::tfidf_preprocess(snippet, stop_words.clone());
  let input_rake_data = preprocess::rake_preprocess(snippet, stop_words.clone());

  // temporary name: case it is the first snippet entry into the database
  if checker {
    sqlite_interface::init(db).await?;
    sqlite_interface::add_document(db, "first document", snippet, input_tfidf_data, input_rake_data).await?;
  } else {
  // cases
  // - if there is a heading then add snippet to a new doc (name will be random)
  // - otherwise put it through the scoring system to find a home, or create new one
  // - both of these will have two cases too
  //  - if there is a document with a score that meets the threshold then store the rake data
  //  into that doc and store the tf_idf as usual for the whole corpus linking to that doc, storing
  //  the snippet there too.
  //  - Otherwise, create the new document with that snippet as the first entry, corpus data will
  //  link to that one instead.
  //  This might be simple as they are stored in a similar way in respecive tables for the data,
  //  with the document as a foreign key
      
//    let data = get_test_corpus();
//    let corpus_tfidf_data = preprocess::corpus_tfidf_preprocess(data.clone(), stop_words.clone());
//    let corpus_rake_data = preprocess::corpus_rake_preprocess(data, stop_words.clone());

    let corpus_tfidf_data = sqlite_interface::load_tfidf_data(db).await?;
    let corpus_rake_data = sqlite_interface::load_rake_data(db).await?;

    // temporary name: case there is a title or the first line has the users chosen prefix in the
    // snippet, then just create a new document
    let check = false;

    if check {
      println!("Found title");
      let title = "test";
      sqlite_interface::add_snippet(db, snippet, title).await?;
      sqlite_interface::update_tfidf_data(db,input_tfidf_data, title).await?;
      sqlite_interface::update_rake_data(db, input_rake_data, title).await?;
    } else {
      let scores = combined_similarity_scores(input_tfidf_data.clone(), input_rake_data.clone(), corpus_tfidf_data, corpus_rake_data, COSINE_WEIGHT);

      if scores[0].1 >= THESHOLD {
        println!("{} is the chosen document with a score of {}", scores[0].0, scores[0].1);

        sqlite_interface::add_snippet(db, snippet, &scores[0].0).await?;
        sqlite_interface::update_tfidf_data(db, input_tfidf_data, &scores[0].0).await?;
        sqlite_interface::update_rake_data(db, input_rake_data, &scores[0].0).await?;
      } else {
        println!("{} doesn't meet the threshold with a score of {}", scores[0].0, scores[0].1);
        println!("Creating new document");
      }
    }
  }

  Ok(())
}

fn combined_similarity_scores(input_tfidf_data: Vec<String>, input_rake_data: Vec<String>, corpus_tfidf_data: CorpusSnippets, corpus_rake_data: CorpusSnippets, cosine_weight: f32) -> Vec<(String, f32)> {
  let corpus_tfidf_scores = tf_idf::corpus_tf_idf_hash(corpus_tfidf_data.clone());
  let corpus_rake_scores = rake::corpus_rake(corpus_rake_data.clone());

  let tf_idf_input_score = tf_idf::tf_idf_hash(input_tfidf_data, corpus_tfidf_data);
  let rake_input_score = rake::rake(input_rake_data.clone());

  let documents_1: HashSet<&str> = corpus_tfidf_scores.keys().map(|k| k.as_str()).collect();
  let documents_2: HashSet<&str> = corpus_rake_scores.keys().map(|k| k.as_str()).collect();
  let all_documents: HashSet<&str> = documents_1.union(&documents_2).map(|v| v.to_owned()).collect();

  let mut combined_scores: HashMap<String, f32> = HashMap::new();

  for document in all_documents {
    let cosine_similarity_score = 
      similarity::cosine_similarity_tuple(tf_idf_input_score.clone(), corpus_tfidf_scores[document].clone())
      * cosine_weight;
    
    let weighted_jaccard_similarity_score = 
      similarity::weighted_jaccard_similarity(input_rake_data.clone(), corpus_rake_data[document].clone(), rake_input_score.clone(), corpus_rake_scores[document].clone())
      * (1. - cosine_weight);
    
    combined_scores.insert(
      document.to_string(),
      cosine_similarity_score + weighted_jaccard_similarity_score
    );
  }

  let mut sorted_scores: Vec<(String, f32)> = combined_scores.into_iter().collect();
  sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  sorted_scores
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

