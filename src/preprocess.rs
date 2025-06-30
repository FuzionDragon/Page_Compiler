use rust_stemmers::{ Algorithm, Stemmer };
use stop_words::{ get, LANGUAGE };
use human_regex::{ one_or_more, punctuation };

pub fn preprocess(data: Vec<String>) -> Vec<Vec<String>> {
  println!("{:?}", data.clone());
  let mut processed = Vec::new();
  let stop_words = get(LANGUAGE::English);
  let en_stemmer = Stemmer::create(Algorithm::English);

  for text in data {
    let lowercase_text = text.to_ascii_lowercase();
    let punctuation_regex = one_or_more(punctuation());
    let no_punctuation_text = punctuation_regex
      .to_regex()
      .replace_all(&lowercase_text, "");

    let clean_text: Vec<String> = no_punctuation_text
      .split_whitespace()
      .filter(|word| !stop_words.contains(&word.to_string()))
      .map(|word| en_stemmer.stem(word).to_string())
      .collect();

    processed.push(clean_text);
  }

  processed
}
