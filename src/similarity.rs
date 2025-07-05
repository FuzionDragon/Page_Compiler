use std::collections::{HashMap, HashSet};

use super::rake::{ rake, all_rake };
use super::tf_idf::{ tf_idf_hash, all_tf_idf_hash };

pub fn cosine_similarity(vec_a: Vec<f32>, vec_b: Vec<f32>) -> f32 {
  let dot_product = vec_a.clone()
    .iter()
    .zip(vec_b.clone().iter())
    .map(|(a, b)| a * b)
    .sum::<f32>();

  let magnitude_a = vec_a.iter()
    .map(|a| a.powi(2))
    .sum::<f32>()
    .sqrt();
  
  let magnitude_b = vec_b.iter()
    .map(|b| b.powi(2))
    .sum::<f32>()
    .sqrt();

  if magnitude_a == 0. || magnitude_b == 0. {
    0.
  } else {
    dot_product / (magnitude_a * magnitude_b)
  }
}

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

pub fn jaccards_similarity(document_1: Vec<String>, document_2: Vec<String>) -> f32 {
  let words_1: HashSet<String> = document_1.into_iter().collect();
  let words_2: HashSet<String> = document_2.into_iter().collect();

  let shared_words = words_1.intersection(&words_2).count() as f32;
  let all_words = words_1.union(&words_2).count() as f32;

  shared_words / all_words
}

pub fn weighted_jaccard_similarity(document_1: Vec<String>, document_2: Vec<String>, document_1_scores: Vec<(String, f32)>, document_2_scores: Vec<(String, f32)>) -> f32 {
  let scores_1: HashMap<String, f32> = HashMap::from_iter(document_1_scores);
  let scores_2: HashMap<String, f32> = HashMap::from_iter(document_2_scores);
  let mut all_words: HashSet<String> = document_1.clone().into_iter().collect();
  all_words.extend(document_2.clone().into_iter().collect::<HashSet<String>>());

  let minimum = all_words.clone().into_iter()
    .map(|k| scores_1.get(&k.to_string())
      .unwrap_or(&0.)
      .min(
        *scores_2.get(&k.to_string())
        .unwrap_or(&0.)
      )
    ).sum::<f32>();

  let maximum = all_words.into_iter()
    .map(|k| scores_1.get(&k.to_string())
      .unwrap_or(&0.)
      .max(
        *scores_2.get(&k.to_string())
        .unwrap_or(&0.)
      )
    ).sum::<f32>();

  minimum / maximum
}

pub fn combined_similarity_scores(input_tfidf_data: Vec<String>, input_rake_data: Vec<String>, corpus_tfidf_data: Vec<Vec<String>>, corpus_rake_data: Vec<Vec<String>>, cosine_weight: f32) -> Vec<(usize, f32)> {
  let all_tfidf_scores = all_tf_idf_hash(corpus_tfidf_data.clone());
  let all_rake_scores = all_rake(corpus_rake_data.clone());

  let tf_idf_input_score = tf_idf_hash(input_tfidf_data, corpus_tfidf_data);
  let rake_input_score = rake(input_rake_data.clone());

  let corpus_1: HashSet<usize> = all_rake_scores.keys().map(|k| k.to_owned()).collect();
  let corpus_2: HashSet<usize> = all_rake_scores.keys().map(|k| k.to_owned()).collect();
  let corpus: HashSet<usize> = corpus_1.union(&corpus_2).map(|v| v.to_owned()).collect();

  let mut combined_scores: HashMap<usize, f32> = HashMap::new();

  for document in corpus {
    let cosine_similarity_score = 
      cosine_similarity_tuple(tf_idf_input_score.clone(), all_tfidf_scores[&document].clone())
      * cosine_weight;
    
    let weighted_jaccard_similarity_score = 
      weighted_jaccard_similarity(input_rake_data.clone(), corpus_rake_data[document].clone(), rake_input_score.clone(), all_rake_scores[&document].clone())
      * (1. - cosine_weight);
    
    combined_scores.insert(
      document,
      cosine_similarity_score + weighted_jaccard_similarity_score
    );
  }

  let mut sorted_scores: Vec<(usize, f32)> = combined_scores.into_iter().collect();
  sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  sorted_scores
}
