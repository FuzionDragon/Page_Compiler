use std::collections::{HashMap, HashSet};

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
  let mut all_words: HashSet<String> = document_1.into_iter().collect();
  all_words.extend(document_2.into_iter().collect::<HashSet<String>>());

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
