use std::iter::Sum;

pub fn square_of_sum(num: u32) -> u32 {
  u32::pow(u32::sum(1..num + 1), 2)
}

pub fn sum_of_squares(num: u32) -> u32 {
  (1..num + 1).fold(0, |acc, n| acc + u32::pow(n, 2))
}

pub fn difference(num: u32) -> u32 {
  square_of_sum(num) - sum_of_squares(num)
}
