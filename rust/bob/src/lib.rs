pub fn reply(say: &str) -> String {
    if say.is_empty() {
        "Fine. Be that way!".to_owned()
    } else if say.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        "Whoa, chill out!".to_owned()
    } else if say.ends_with("!") {
        "Whatever.".to_owned()
    } else if say.ends_with("?") {
        "Sure.".to_owned()
    } else {
        "Whatever.".to_owned()
    }
}
