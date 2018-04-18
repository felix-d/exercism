use std::collections::HashSet;

pub fn check(string: &str) -> bool {
    let mut counts = HashSet::new();

    string.to_lowercase().chars().all(|c| {
        if c.is_whitespace() || c == '-' {
            true
        } else if !counts.contains(&c) {
            counts.insert(c);
            true
        } else {
            false
        }
    })
}
