pub fn cosine_similarity(vec_a: Vec<f32>, vec_b: Vec<f32>) -> f32 {
  let dot_product = vec_a.clone()
    .iter()
    .zip(vec_b.clone().iter())
    .map(|(a, b)| a * b)
    .sum::<f32>();

  let magnitude_a = vec_a.iter()
    .map(|a| a * a)
    .sum::<f32>()
    .sqrt();
  
  let magnitude_b = vec_b.iter()
    .map(|b| b * b)
    .sum::<f32>()
    .sqrt();

  dot_product / (magnitude_a * magnitude_b)
}

pub fn jaccards_similarity() {}
