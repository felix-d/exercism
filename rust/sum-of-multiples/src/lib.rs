use std::iter::Sum;

pub fn sum_of_multiples(ceiling: u32, numbers: &Vec<u32>) -> u32 {
  (1..ceiling).fold(0, |acc, n|
    if numbers.iter().any(|x| n % x == 0) { acc + n } else { acc }
  )
}
