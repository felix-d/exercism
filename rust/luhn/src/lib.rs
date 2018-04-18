pub fn is_valid(input: &str) -> bool {
  if input.chars().filter(|c| c.is_numeric()).count() == 1 ||
    !input.chars().all(|c| c.is_whitespace() || c.is_numeric()) {
    return false
  }

  input
    .chars()
    .filter(|c| c.is_numeric())
    .map(|c| c.to_digit(10).unwrap())
    .rev()
    .enumerate()
    .map(|(i, c)| {
      if i % 2 == 0 { return c }
      let result = c * 2;
      if result > 9 { result - 9 } else { result }
    })
    .sum::<u32>() % 10 == 0
}
