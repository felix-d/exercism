pub fn sing(from: u32, to: u32) -> String {
  let mut verse = (to..from + 1)
    .map(|verse_num| verse(verse_num))
    .fold(String::new(), |acc, string| format!("{}\n{}", string, acc));
  verse.pop();
  verse
}

pub fn verse(verse_num: u32) -> String {
  match verse_num {
    0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
       Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
    1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
       Take it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
    2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
       Take one down and pass it around, 1 bottle of beer on the wall.\n".to_owned(),
    _ => build_verse(verse_num)
  }
}

fn build_verse(verse_num: u32) -> String {
  format!(
    "{current} bottles of beer on the wall, {current} bottles of beer.\n\
    Take one down and pass it around, {next} bottles of beer on the wall.\n",
    current = verse_num,
    next = verse_num - 1,
  )
}
