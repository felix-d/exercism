extern crate rayon;

use std::collections::HashMap;
use rayon::prelude::*;

const ERR: &'static str = "Invalid nucleotide";

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, &'static str> {
  if !is_valid(dna) { return Err(ERR) }

  let (a, c, g, t) = dna.par_chars().fold(|| (0, 0, 0, 0), |(a, c, g, t), nuc| {
    match nuc {
      'A' => (a + 1, c, g, t),
      'C' => (a, c + 1, g, t),
      'G' => (a, c, g + 1, t),
      'T' => (a, c, g, t + 1),
      _ => (a, c, g, t)
    }
  }).reduce(|| (0, 0, 0, 0), |(a_sum, c_sum, g_sum, t_sum), (a, c, g, t)| {
    (a_sum + a, c_sum + c, g_sum + g, t_sum + t)
  });

  Ok(vec![('A', a), ('C', c), ('G', g), ('T', t)].into_iter().collect())
}

pub fn count(nuc: char, dna: &str, ) -> Result<u64, &'static str> {
  if !is_valid(nuc.to_string().as_str()) || !is_valid(dna) { return Err(ERR) }

  Ok(dna.par_chars().fold(|| 0, |sum, n| if nuc == n { sum + 1 } else { sum }).sum())
}

fn is_valid(dna: &str) -> bool {
  dna.par_chars().all(|c| "ACGT".contains(c))
}
