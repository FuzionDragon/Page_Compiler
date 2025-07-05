use std::collections::{HashMap, HashSet};

// the document isnt made up of single words but phrases
pub fn rake(document: Vec<String>) -> Vec<(String, f32)> {
  let mut words: Vec<String> = Vec::new();

  for phrase in document.clone() {
    words.extend(phrase.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>());
  }

  let unique_words: HashSet<String> = HashSet::from_iter(words);

  let word_degrees = word_degrees(document.clone(), unique_words);
  let word_frequency = word_frequency(document.clone());

  let degree_scores = degree_scores(word_degrees, word_frequency);
  let mut scores = phrase_degree_scores(document, degree_scores.clone());
  scores.extend(degree_scores);

  let mut vec_scores: Vec<(String, f32)> = scores.into_iter().collect();
  vec_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  vec_scores
}

pub fn all_rake(corpus: Vec<Vec<String>>) -> HashMap<usize, Vec<(String, f32)>> {
  let mut all_rake_scores: HashMap<usize, Vec<(String, f32)>> = HashMap::new();
  for (i, document) in corpus.iter().enumerate() {
    all_rake_scores.insert(i, rake(document.clone()));
  }

  all_rake_scores
}

fn degree_scores(degree_of_words: HashMap<String, f32>, word_frequency: HashMap<String, f32>) -> HashMap<String, f32> {
  let mut degree_scores: HashMap<String, f32> = HashMap::new();

  for word in degree_of_words.clone().into_keys() {
    degree_scores.insert(word.clone(), degree_of_words[&word] / word_frequency[&word]);
  }

  degree_scores
}

fn word_frequency(document: Vec<String>) -> HashMap<String, f32> {
  let mut frequencies: HashMap<String, f32> = HashMap::new();
  let mut tokened_document: Vec<String> = Vec::new();

  for phrase in document {
    tokened_document.extend(phrase.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>());
  }

  for word in tokened_document {
    *frequencies.entry(word).or_default() += 1.;
  }

  frequencies
}

fn word_degrees(phrases: Vec<String>, words: HashSet<String>) -> HashMap<String, f32> {
  let mut word_degrees: HashMap<String, f32> = HashMap::new();

  for phrase in phrases {
    let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
    for phrase_word in phrase_words.clone() {
      if words.contains(phrase_word) {
        *word_degrees.entry(phrase_word.to_string()).or_default() += phrase_words.len() as f32;
      }
    }
  }

  word_degrees
}

fn phrase_degree_scores(phrases: Vec<String>, degree_scores: HashMap<String, f32>) -> HashMap<String, f32> {
  let mut score: HashMap<String, f32> = HashMap::new();

  for phrase in phrases {
    let phrase_words = phrase.split_whitespace();
    for phrase_word in phrase_words {
      let degree_score = degree_scores.get(phrase_word).unwrap_or(&0.);
      *score.entry(phrase.to_string()).or_default() += degree_score;
    }
  }

  score
}
