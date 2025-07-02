use std::collections::HashMap;

use stop_words::{ get, LANGUAGE };

// Needed functions:
// - phrase compiler
// - degree of word calculator (number of times they occur in a phrase as well as number of words
// in their phrase)
// - degree score calculator (degree of word / word frequency)
// - phrase score calculator (all word scores in the phrases added together)

pub fn rake(document: Vec<String>) -> Vec<(String, f32)> {
  let stop_words: Vec<String> = get(LANGUAGE::English);
  let phrases = phrases(document.clone(), stop_words.clone());
  let words: Vec<String> = document.clone()
    .into_iter()
    .filter(|word| !stop_words.contains(&word.to_string()))
    .collect();

  let word_degrees = word_degrees(phrases.clone(), words);
  let word_frequency = word_frequency(document.clone(), stop_words);

  let degree_scores = degree_scores(word_degrees, word_frequency);
  let mut scores = phrase_degree_scores(phrases, degree_scores.clone());
  scores.extend(degree_scores);

  let mut vec_scores: Vec<(String, f32)> = scores.into_iter().collect();
  vec_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  vec_scores
}

fn degree_scores(degree_of_words: HashMap<String, f32>, word_frequency: HashMap<String, f32>) -> HashMap<String, f32> {
  let mut degree_scores: HashMap<String, f32> = HashMap::new();

  for word in degree_of_words.clone().into_keys() {
    degree_scores.insert(word.clone(), degree_of_words[&word] / word_frequency[&word]);
  }

  degree_scores
}

fn word_frequency(document: Vec<String>, stop_words: Vec<String>) -> HashMap<String, f32> {
  let mut frequencies: HashMap<String, f32> = HashMap::new();
  let cleaned_document: Vec<String> = document
    .into_iter()
    .filter(|word| !stop_words.contains(&word.to_string()))
    .collect();

  for word in cleaned_document {
    *frequencies.entry(word).or_default() += 1.;
  }

  frequencies
}

fn word_degrees(phrases: Vec<String>, words: Vec<String>) -> HashMap<String, f32> {
  let mut word_degrees: HashMap<String, f32> = HashMap::new();

  for phrase in phrases {
    let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
    for phrase_word in phrase_words.clone() {
      if words.contains(&phrase_word.to_string()) {
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
      *score.entry(phrase.to_string()).or_default() += degree_scores[phrase_word];
    }
  }

  score
}

fn phrases(document: Vec<String>, stop_words: Vec<String>) -> Vec<String> {
  let mut phrases: Vec<String> = Vec::with_capacity(document.len());
  let mut phrase: Vec<String> = Vec::new();
  for word in document {
    if stop_words.contains(&word) && !phrase.is_empty() {
      phrases.push(phrase.clone().join(" "));
      phrase.clear();
    } else {
      phrase.push(word);
    }
  }
  if !phrase.is_empty() {
    phrases.push(phrase.join(" "));
  }

  phrases
}
