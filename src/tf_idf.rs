use std::collections::{HashMap, HashSet};

pub fn tf_idf(term: &str, document: Vec<&str>, corpus: Vec<Vec<&str>>) -> f32 {
  tf(term, document) * idf(term, corpus)
}

pub fn tf_idf_vectorize(new_document: Vec<&str>, corpus: Vec<Vec<&str>>) -> Vec<f32> {
  let mut vector: Vec<f32> = Vec::new();
  let mut all_terms: HashSet<&str> = HashSet::new();

  for old_document in corpus.clone() {
    all_terms.extend(old_document);
  }

  for term in all_terms {
    vector.push(tf_idf(term, new_document.clone(), corpus.clone()));
  }

  vector
}

pub fn tf_idf_hash(new_document: Vec<&str>, corpus: Vec<Vec<&str>>) -> HashMap<String, f32> {
  let mut scores: HashMap<String, f32> = HashMap::new();
  let mut all_terms: HashSet<&str> = HashSet::new();

  for old_document in corpus.clone() {
    all_terms.extend(old_document);
  }

  for term in all_terms {
    scores.insert(term.to_string(), tf_idf(term, new_document.clone(), corpus.clone()));
  }

  scores
}

pub fn all_tf_idf_vectorize(corpus: Vec<Vec<&str>>) -> HashMap<usize, Vec<f32>> {
  let mut vectors: HashMap<usize, Vec<f32>> = HashMap::new();
  let mut all_terms: HashSet<&str> = HashSet::new();

  for document in corpus.clone() {
    all_terms.extend(document);
  }

  for i in 0..corpus.clone().len() {
    let mut vector: Vec<f32> = Vec::new();

    for term in all_terms.clone() {
      vector.push(tf_idf(term, corpus[i].clone(), corpus.clone()));
    }

    vectors.insert(i, vector);
  }

  vectors
}

pub fn all_tf_idf_hash(corpus: Vec<Vec<&str>>) -> HashMap<usize, HashMap<String, f32>> {
  let mut hashes: HashMap<usize, HashMap<String, f32>> = HashMap::new();
  let mut all_terms: HashSet<&str> = HashSet::new();

  for document in corpus.clone() {
    all_terms.extend(document);
  }

  for i in 0..corpus.clone().len() {
    let mut vector: Vec<f32> = Vec::new();

    for term in all_terms.clone() {
      vector.push(tf_idf(term, corpus[i].clone(), corpus.clone()));
    }

    hashes.insert(i, tf_idf_hash(corpus[i].clone(), corpus.clone()));
  }

  hashes
}

fn tf(search_term: &str, document: Vec<&str>) -> f32 {
  let mut search_term_count = 0.;
  let mut all_term_count = 0.;

  for term in document {
    if term == search_term {
      search_term_count += 1.;
    }

    all_term_count += 1.;
  }

  search_term_count / all_term_count
}

fn idf(term: &str, corpus: Vec<Vec<&str>>) -> f32 {
  let mut count: f32 = 0.;
  let mut total_documents: f32 = 0.;

  for document in corpus {
    if document.contains(&term) {
      count += 1.;
    }

    total_documents += 1.;
  };

  (total_documents / count).ln()
}
