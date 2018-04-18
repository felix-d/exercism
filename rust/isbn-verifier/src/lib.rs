#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub fn is_valid_isbn(isbn: &str) -> bool {
    lazy_static! {
        static ref FORMAT: Regex = Regex::new(r"^(\d-\d{3}-\d{5}-[\dX]|\d{9}[\dX])$").unwrap();
    }

    if !FORMAT.is_match(isbn) { return false; }

    (1..=10).rev().zip(isbn.chars().filter(|c| c.is_alphanumeric())).fold(0, |acc, (mult, c)| {
        acc + match c {
            'X' => mult * 10,
            c => mult * c.to_digit(10).unwrap()
        }
    }) % 11 == 0
}
