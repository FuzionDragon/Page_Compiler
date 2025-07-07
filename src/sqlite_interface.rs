use std::collections::HashMap;

use anyhow::{ Ok, Result };
use sqlx::{sqlite, FromRow, SqlitePool};

type Corpus = Vec<Document>;
type Document = HashMap<String, String>;

#[derive(Debug, FromRow, Clone)]
pub struct Snippet {
  text: String,
  document: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct Term {
  term: String,
  tfidf_score: f32,
  rake_score: f32,
}

pub async fn init(db: &SqlitePool) -> Result<()> {
  sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS Term (
      term TEXT PRIMARY KEY UNIQUE NOT NULL,
      tfidf_score REAL,
      rake_score REAL
    );
  "#).execute(db)
    .await?;

  sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS Document (
      document_id INTEGER PRIMARY KEY AUTOINCREMENT,
      document_name TEXT
    );
  "#).execute(db)
    .await?;

  sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS Snippet (
      snippet_id INTEGER PRIMARY KEY AUTOINCREMENT,
      snippet TEXT NOT NULL,
      document_id INTEGER,
      FOREIGN KEY (document_id)
        REFERENCES Document (document_id)
    );
  "#).execute(db)
    .await?;

  Ok(())
}

pub async fn load_snippets(db: &SqlitePool) -> Result<Vec<Snippet>> {
  let query = r#"
    SELECT Document.document_name, snippet FROM Snippet
    LEFT JOIN Document ON Snippet.document_id == Document.document_id;
  "#;

  let snippets = sqlx::query_as::<_, Snippet>(query)
    .fetch_all(db)
    .await?;

  Ok(snippets)
}

pub async fn load_documents(db: &SqlitePool) -> Result<Document> {
  let snippets = load_snippets(db).await?;

  let mut documents: Document = HashMap::new();

  for snippet in snippets {
    documents
      .entry(snippet.document.clone())
      .and_modify(|v| v.push_str("\n\n{snippet.text.clone()}"))
      .or_insert(snippet.text.clone());
  }

  Ok(documents)
}

pub async fn load_terms(db: &SqlitePool) -> Result<Vec<Term>> {
  let query = r#"
    SELECT * FROM Term;
  "#;

  let terms = sqlx::query_as::<_, Term>(query)
    .fetch_all(db)
    .await?;

  Ok(terms)
}

pub async fn add_snippets(db: &SqlitePool, snippets: Vec<Snippet>) -> Result<()> {
  for snippet in snippets {
    sqlx::query("INSERT INTO Snippets (text, document_name) VALUES ($1)")
      .bind(&snippet.text)
      .bind(&snippet.document)
      .execute(db)
      .await?;
  }

  Ok(())
}

pub async fn add_terms(db: &SqlitePool, terms: Vec<Term>) -> Result<()> {
  for term in terms {
    sqlx::query("INSERT INTO Term (term, tfidf_score, rake_score) VALUES ($1, $2, $3)")
      .bind(term.term)
      .bind(term.tfidf_score)
      .bind(term.rake_score)
      .execute(db)
      .await?;
  }
  Ok(())
}

pub async fn update_tfidf_scores(db: &SqlitePool, new_tfidf_scores: HashMap<String, f32>) -> Result<()> {
  for (term, score) in new_tfidf_scores {
  }
//  let directories  = sqlx::query_as::<_, Snippet>("SELECT * FROM Snippets WHERE dir==$1")
//    .bind(&new_dir)
//    .fetch_all(db)
//    .await?;
//
//  if directories.is_empty() {
//    sqlx::query(r#"
//      UPDATE Snippets
//      SET dir=$1
//      WHERE name=$2;
//      "#).bind(new_dir)
//      .bind(name)
//      .execute(db)
//      .await?;
//  }

  Ok(())
}

pub async fn clear(db: &SqlitePool) -> Result<()> {
  sqlx::query("DELETE FROM Snippets").execute(db)
    .await?;
    
  Ok(())
}

pub async fn query_name(db: &SqlitePool, name: String) -> Result<Snippet> {
  let found_special = sqlx::query_as::<_, Snippet>("SELECT * FROM Snippets WHERE name==$1")
    .bind(&name)
    .fetch_one(db)
    .await?;

  Ok(found_special)
}
