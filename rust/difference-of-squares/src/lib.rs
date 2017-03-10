pub fn sum_of_squares(nat: i32) -> i32 {
    (1..nat + 1).fold(0, |sum, x| sum + x.pow(2))
}

pub fn square_of_sum(nat: i32) -> i32 {
    (1..nat + 1).fold(0, |sum, x| sum + x).pow(2)
}

pub fn difference(nat: i32) -> i32 {
    square_of_sum(nat) - sum_of_squares(nat)
}
