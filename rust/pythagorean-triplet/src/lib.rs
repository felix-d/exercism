pub fn find() -> Option<u32> {
    const N: u32 = 1000;

    for x in 1..N - 1 {
        let x_sq = x.pow(2);
        for y in 1..N - x {
            let z = 1000 - x - y;
            if x_sq + y.pow(2) == z.pow(2) {
                return Some(x * y * z);
            }
        }
    }

    None
}
