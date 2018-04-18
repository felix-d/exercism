pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::new(); digits.len() + 1];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|window| window.iter().collect())
        .collect()
}
