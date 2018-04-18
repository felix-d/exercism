extern crate regex;

pub fn lsp(digits: &str, span: usize) -> Result<u32, ()> {
  if digits.len() < span || !digits.chars().all(char::is_numeric) { return Err(()) }
  if span == 0 { return Ok(1) }

  digits
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect::<Vec<u32>>()
    .windows(span)
    .map(|win| win.iter().product())
    .max()
    .ok_or(())
}
