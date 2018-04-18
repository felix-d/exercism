pub fn hamming_distance(nuc_a: &str, nuc_b: &str) -> Result<usize, &'static str> {
  if nuc_a.len() != nuc_b.len() { return Err("Lengths must be equal.") }

  Ok(nuc_a.chars().zip(nuc_b.chars()).filter(|&(a, b)| a != b).count())
}
