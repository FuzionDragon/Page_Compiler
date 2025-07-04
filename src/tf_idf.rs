use std::collections::{HashMap, HashSet};

pub fn tf_idf(term: String, document: Vec<String>, corpus: Vec<Vec<String>>) -> f32 {
  tf(term.clone(), document) * idf(term.clone(), corpus)
}

pub fn tf_idf_vectorize(new_document: Vec<String>, corpus: Vec<Vec<String>>) -> Vec<f32> {
  let mut vector: Vec<f32> = Vec::new();
  let mut all_terms: HashSet<String> = HashSet::new();

  for old_document in corpus.clone() {
    all_terms.extend(old_document);
  }

  for term in all_terms {
    vector.push(tf_idf(term.clone(), new_document.clone(), corpus.clone()));
  }

  vector
}

pub fn all_tf_idf_vectorize(corpus: Vec<Vec<String>>) -> HashMap<usize, Vec<f32>> {
  let mut vectors: HashMap<usize, Vec<f32>> = HashMap::new();
  let mut all_terms: HashSet<String> = HashSet::new();

  for document in corpus.clone() {
    all_terms.extend(document);
  }

  for i in 0..corpus.clone().len() {
    let mut vector: Vec<f32> = Vec::new();

    for term in all_terms.clone() {
      vector.push(tf_idf(term.clone(), corpus[i].clone(), corpus.clone()));
    }

    vectors.insert(i, vector);
  }

  vectors
}

pub fn compute_all_idf(corpus: Vec<Vec<String>>) -> HashMap<String, f32> {
  let mut all_idf: HashMap<String, f32> = HashMap::new();
  let mut all_terms: HashSet<String> = HashSet::new();

  for document in corpus.clone() {
    all_terms.extend(document);
  }

  for term in all_terms {
    all_idf.insert(term.clone(), idf(term.clone(), corpus.clone()));
  }

  all_idf
}

fn tf(search_term: String, document: Vec<String>) -> f32 {
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

fn idf(term: String, corpus: Vec<Vec<String>>) -> f32 {
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
