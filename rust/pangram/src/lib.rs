#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).fold(0_u32, |bitmap, c| {
      bitmap | 1_u32 << ((c as u32) - ('A' as u32))
    }) == u32::pow(2, 26) - 1
}

