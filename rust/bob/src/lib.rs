pub fn reply(question: &'static str) -> &'static str {
  let question = question.trim();

  if question.is_empty() {
    "Fine. Be that way!"
  } else if is_yelling(question) {
    "Whoa, chill out!"
  } else if question.chars().last() == Some('?') {
    "Sure."
  } else {
    "Whatever."
  }
}

fn is_yelling(question: &str) -> bool {
  let alphabetic = question.trim_matches(|c: char| !c.is_alphabetic());
  !alphabetic.is_empty() && alphabetic == alphabetic.to_uppercase()
}
