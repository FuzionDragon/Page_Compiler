use stop_words::{ get, LANGUAGE };
use human_regex::{ exactly, one_or_more, or, punctuation, whitespace, word_boundary };

pub fn preprocess(data: Vec<String>) -> Vec<String> {
  println!("{:?}", data.clone());
  let mut processed = Vec::new();
  let stop_words = get(LANGUAGE::English);

  for text in data {
    let lowercase_text = text.to_ascii_lowercase();
    let punctuation_regex = one_or_more(punctuation());
    let no_punctuation_text = punctuation_regex
      .to_regex()
      .replace_all(&lowercase_text, "");

    let stop_words_regex = word_boundary() + exactly(1, or(&stop_words)) + word_boundary() + one_or_more(whitespace());
    let clean_text = stop_words_regex
      .to_regex()
      .replace_all(&no_punctuation_text, "")
      .to_string();

    processed.push(clean_text);
  }

  processed
}
