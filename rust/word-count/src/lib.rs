#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate itertools;
extern crate num_cpus;
extern crate rayon;

use rayon::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;


pub fn word_count(input: &str) -> HashMap<String, u32> {
  let chunks: Vec<String> = get_chunks(input);

  chunks.into_par_iter()
    .map(|chunk| count_words(chunk))
    .reduce(|| HashMap::new(), |mut result, word_count| {
      merge_maps(&mut result, word_count);
      result
    })
}

fn merge_maps(to: &mut HashMap<String, u32>, from: HashMap<String, u32>) {
  for (key, val) in from.iter() {
    let count = to.entry(key.to_owned()).or_insert(0);
    *count += *val;
  }
}

fn count_words(chunk: String) -> HashMap<String, u32> {
  chunk
    .split_whitespace()
    .map(|word| word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect())
    .fold(HashMap::new(), |mut res, word: String| {
      if !word.is_empty() {
        *res.entry(word.to_owned()).or_insert(0) += 1
      }
      res
    })
}

fn get_chunks(input: &str) -> Vec<String> {
  let len: usize = input.len();
  let cpus: usize = num_cpus::get();
  let step = if len / cpus == 0 { len } else { len / cpus };

  input.chars().batching(|it| {
    let chunk = it.clone().take(step);
    let tail = it.skip(step).take_while(|c| !c.is_whitespace());
    let full_chunk: String = chunk.chain(tail).collect();

    if full_chunk.is_empty() { None } else { Some(full_chunk) }
  }).collect()
}
