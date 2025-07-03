use anyhow::{ Ok, Result };
use sqlx::{SqlitePool, FromRow, Row, Error::Database};

#[derive(Debug, FromRow, Clone)]
pub struct Snippet {
  snippet_id: i32,
  text: String,
  category: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct Category {
  catergory_id: i32,
  category_name: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct Term {
  term_id: i32,
  term: String,
}

pub async fn init(db: &SqlitePool) -> Result<()> {
  sqlx::query(r#"
    create table if not exists Terms (
      term_id integer primary key autoincrement,
      term text unique not null
      );
  "#).execute(db)
    .await?;

  sqlx::query(r#"
    create table if not exists Categories (
      category_id integer primary key autoincrement,
      category_name text
      );
  "#).execute(db)
    .await?;

  sqlx::query(r#"
    create table if not exists Snippets (
      snippet_id integer primary key autoincrement,
      snippet text not null,
      category_id integer 
    );
  "#).execute(db)
    .await?;

  Ok(())
}

pub async fn load(db: &SqlitePool) -> Result<Vec<Snippet>> {
  let snippets = sqlx::query_as::<_, Snippet>("SELECT * FROM Snippets")
    .fetch_all(db)
    .await?;

  Ok(snippets)
}

// Used by hook_project and mark_project to add 
pub async fn add(db: &SqlitePool, text: String) -> Result<()> {
  sqlx::query("INSERT INTO Snippets (text) VALUES ($1)")
    .bind(&text)
    .execute(db)
    .await?;

  Ok(())
}

/// Updates a specific project directory, used by Mark
pub async fn update_directory(db: &SqlitePool, name: String, new_dir: String) -> Result<()> {
  let directories  = sqlx::query_as::<_, Snippet>("SELECT * FROM Snippets WHERE dir==$1")
    .bind(&new_dir)
    .fetch_all(db)
    .await?;

  if directories.is_empty() {
    sqlx::query(r#"
      UPDATE Snippets
      SET dir=$1
      WHERE name=$2;
      "#).bind(new_dir)
      .bind(name)
      .execute(db)
      .await?;
  }

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
