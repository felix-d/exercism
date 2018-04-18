pub fn is_armstrong_number(num: u32) -> bool {
    let digits_str = num.to_string();
    let len = digits_str.len() as u32;
    let sum: u32 = digits_str.chars().map(|c| c.to_digit(10).unwrap().pow(len)).sum();
    sum == num
}
