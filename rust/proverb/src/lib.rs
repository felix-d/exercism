pub fn build_proverb(input: Vec<&str>) -> String {
  match input.len() {
    0 => "".to_owned(),
    1 => "And all for the want of a nail.".to_owned(),
    2 => format!("{}\nAnd all for the want of a nail.", prefix(input)),
    _ => format!("{}\nAnd all for the want of a horseshoe nail.", prefix(input)),
  }
}

fn prefix(input: Vec<&str>) -> String {
  input[..input.len()]
    .windows(2)
    .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]).to_owned())
    .collect::<Vec<String>>()
    .join("\n")
}
