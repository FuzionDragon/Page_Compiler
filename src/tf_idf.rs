use std::collections::{HashMap, HashSet};

pub fn tf_idf(term: String, document: Vec<String>, corpus: Vec<Vec<String>>) -> f32 {
  tf(term.clone(), document) * idf(term.clone(), corpus)
}

pub fn all_document_tf_idf(term: String, corpus: Vec<Vec<String>>) -> HashMap<usize, f32> {
  let mut result: HashMap<usize, f32> = HashMap::new();

  for i in 0..corpus.len() {
    let document = corpus[i].clone();
    result.insert(i, tf_idf(term.clone(), document, corpus.clone()));
  }

  result
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

  (total_documents / count).log(10.0)
}
