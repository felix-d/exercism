static SMALL: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static TENS: &[&str] = &[
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

static BIG: &[&str] = &[
    "hundred",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    if n < 20 {
        format!("{}", SMALL[n as usize])
    } else if n < 100 && n % 10 == 0 {
        format!("{}", TENS[n as usize / 10 - 2])
    } else if n < 100 {
        format!("{}-{}", TENS[n as usize / 10 - 2], SMALL[n as usize % 10])
    } else {
        let x = (n as f64).log(1000f64) as u32;
        let z = if x < 1 { 100 } else { 1000u64.pow(x) };

        if n % z == 0 {
            format!("{} {}", encode(n / z), BIG[x as usize])
        } else  {
            format!("{} {} {}", encode(n / z), BIG[x as usize], encode(n % z))
        }
    }
}
