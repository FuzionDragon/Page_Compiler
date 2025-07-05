use rust_stemmers::{ Algorithm, Stemmer };
use human_regex::{ one_or_more, punctuation };

pub fn corpus_tfidf_preprocess(corpus: Vec<String>, stop_words: Vec<String>) -> Vec<Vec<String>> {
  let mut processed = Vec::new();

  for document in corpus {
    processed.push(tfidf_preprocess(document, stop_words.clone()));
  }

  processed
}

// currently only processes a corpus as opposed to a document
pub fn corpus_rake_preprocess(corpus: Vec<String>, stop_words: Vec<String>) -> Vec<Vec<String>> {
  let mut processed = Vec::new();

  for document in corpus {
    processed.push(rake_preprocess(document, stop_words.clone()));
  }

  processed
}

pub fn rake_preprocess(document: String, stop_words: Vec<String>) -> Vec<String> {
  let en_stemmer = Stemmer::create(Algorithm::English);
  let lowercase_text = document.to_ascii_lowercase();
  let punctuation_regex = one_or_more(punctuation());
  let no_punctuation_text = punctuation_regex
    .to_regex()
    .replace_all(&lowercase_text, "");

  let clean_text: Vec<String> = no_punctuation_text
    .split_whitespace()
    .map(|word| en_stemmer.stem(word).to_string())
    .collect();

  let mut phrases: Vec<String> = Vec::with_capacity(document.len());
  let mut phrase: Vec<String> = Vec::new();

  for word in clean_text {
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

  phrases.into_iter()
    .filter(|word| !stop_words.contains(&word.to_string()))
    .collect::<Vec<String>>()
}

pub fn tfidf_preprocess(document: String, stop_words: Vec<String>) -> Vec<String> {
  let en_stemmer = Stemmer::create(Algorithm::English);

  let lowercase_text = document.to_ascii_lowercase();
  let punctuation_regex = one_or_more(punctuation());
  let no_punctuation_text = punctuation_regex
    .to_regex()
    .replace_all(&lowercase_text, "");

  let clean_text: Vec<String> = no_punctuation_text
    .split_whitespace()
    .filter(|word| !stop_words.contains(&word.to_string()))
    .map(|word| en_stemmer.stem(word).to_string())
    .collect();

  clean_text
}
