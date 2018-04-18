#![feature(iterator_step_by)]

pub fn nth(n: u32) -> Option<u32> {
    if n == 0 { return None; }

    let mut primes = vec![2];
    let mut num = 3;

    while (primes.len() as u32) < n {
        if primes.iter().all(|&p| num % p != 0) {
            primes.push(num);
        }
        num += 2;
    }

    primes.last().map(|v| *v)
}
