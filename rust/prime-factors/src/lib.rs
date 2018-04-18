extern crate num_iter;

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut num = n;

    if n < 2 {
        return factors;
    }

    while num != 1 {
        for i in 2..num + 1 {
            if num % i == 0 {
                factors.push(i);
                num /= i;
                break;
            }
        }
    }

    factors
}
