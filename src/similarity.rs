use std::collections::{HashMap, HashSet};

use crate::CorpusSnippets;

use super::rake::{ rake, corpus_rake };
use super::tf_idf::{ tf_idf_hash, corpus_tf_idf_hash };

// Scores are obtained from tfidf
pub fn cosine_similarity_tuple(scores_1: HashMap<String, f32>, scores_2: HashMap<String, f32>) -> f32 {
  let mut paired_scores: Vec<(f32, f32)> = Vec::new();
  let mut terms: HashSet<&String> = scores_1.keys().collect();
  terms.extend::<HashSet<&String>>(scores_2.keys().collect());

  for term in terms {
    paired_scores.push((
      scores_1.get(term).unwrap_or(&0.).to_owned(),
      scores_2.get(term).unwrap_or(&0.).to_owned()
    ));
  }

  let dot_product = paired_scores.clone()
    .iter()
    .map(|v| v.0 * v.1)
    .sum::<f32>();

  let magnitude_a = paired_scores.clone().iter()
    .map(|v| v.0.powi(2))
    .sum::<f32>()
    .sqrt();
  
  let magnitude_b = paired_scores.iter()
    .map(|v| v.1.powi(2))
    .sum::<f32>()
    .sqrt();

  if magnitude_a == 0. || magnitude_b == 0. {
    0.
  } else {
    dot_product / (magnitude_a * magnitude_b)
  }
}

// weights base on RAKE algorithm scores
pub fn weighted_jaccard_similarity(document_1: Vec<String>, document_2: Vec<String>, document_1_scores: HashMap<String, f32>, document_2_scores: HashMap<String, f32>) -> f32 {
  let scores_1: HashMap<String, f32> = HashMap::from_iter(document_1_scores);
  let scores_2: HashMap<String, f32> = HashMap::from_iter(document_2_scores);
  let mut all_words: HashSet<&str> = document_1.iter().map(|v| v.as_str()).collect();
  all_words.extend::<HashSet<&str>>(document_2.iter().map(|v| v.as_str()).collect());

  let minimum = all_words.clone().into_iter()
    .map(|k| scores_1.get(k)
      .unwrap_or(&0.)
      .min(
        *scores_2.get(k)
        .unwrap_or(&0.)
      )
    ).sum::<f32>();

  let maximum = all_words.into_iter()
    .map(|k| scores_1.get(k)
      .unwrap_or(&0.)
      .max(
        *scores_2.get(k)
        .unwrap_or(&0.)
      )
    ).sum::<f32>();

  minimum / maximum
}

pub fn combined_similarity_scores(input_tfidf_data: Vec<String>, input_rake_data: Vec<String>, corpus_tfidf_data: CorpusSnippets, corpus_rake_data: CorpusSnippets, cosine_weight: f32) -> Vec<(String, f32)> {
  let corpus_tfidf_scores = corpus_tf_idf_hash(corpus_tfidf_data.clone());
  let corpus_rake_scores = corpus_rake(corpus_rake_data.clone());

  let tf_idf_input_score = tf_idf_hash(input_tfidf_data, corpus_tfidf_data);
  let rake_input_score = rake(input_rake_data.clone());

  let documents_1: HashSet<&str> = corpus_tfidf_scores.keys().map(|k| k.as_str()).collect();
  let documents_2: HashSet<&str> = corpus_rake_scores.keys().map(|k| k.as_str()).collect();
  let all_documents: HashSet<&str> = documents_1.union(&documents_2).map(|v| v.to_owned()).collect();

  let mut combined_scores: HashMap<String, f32> = HashMap::new();

  for document in all_documents {
    let cosine_similarity_score = 
      cosine_similarity_tuple(tf_idf_input_score.clone(), corpus_tfidf_scores[document].clone())
      * cosine_weight;
    
    let weighted_jaccard_similarity_score = 
      weighted_jaccard_similarity(input_rake_data.clone(), corpus_rake_data[document].clone(), rake_input_score.clone(), corpus_rake_scores[document].clone())
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
