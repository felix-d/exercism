pub fn raindrops(drops: u32) -> String {
  let mut result = String::new();

  if drops % 3 == 0 { result.push_str("Pling"); }
  if drops % 5 == 0 { result.push_str("Plang"); }
  if drops % 7 == 0 { result.push_str("Plong"); }

  match result.as_ref() {
    "" => drops.to_string(),
    _ => result
  }
}
