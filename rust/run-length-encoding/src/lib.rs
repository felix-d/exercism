pub fn encode(string: &str) -> String {
  let (result, last, count): (String, Option<char>, usize) =
    string.chars().fold((String::new(), None, 0), |(result, last, count), c| {
      match last {
        Some(last) if last == c => (result, Some(c), count + 1),
        Some(last) => (encode_char(result, last, count), Some(c), 1),
        None => (result, Some(c), 1),
      }
  });

  if last == None { return String::new() }
  encode_char(result, last.unwrap(), count)
}

fn encode_char(mut result: String, last: char, count: usize) -> String {
  match count {
    1 => result.push(last),
    _ => result.push_str(format!("{}{}", count, last).as_str()),
  }
  result
}

pub fn decode(string: &str) -> String {
  string.chars().fold((String::new(), Vec::<char>::new()), |(mut result, mut stack), c| {
    if c.is_numeric() {
      stack.push(c);
    } else if stack.is_empty() {
      result.push(c);
    } else {
      let segment = decode_segment(&stack, c);
      result.push_str(segment.as_str());
      stack.clear();
    }
    (result, stack)
  }).0
}

fn decode_segment(stack: &Vec<char>, c: char) -> String {
  let repeat_str: String = stack.iter().collect();
  let repeat: usize = repeat_str.parse::<usize>().unwrap();
  (0..repeat).fold(String::new(), |mut result, _| {
    result.push(c);
    result
  })
}
