pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    fn add_numbers(&self, last: Vec<u32>, i: u32) -> Vec<u32> {
        let mut inner: Vec<u32> = Vec::new();

        for j in 0..i {
            let bef = if j > 0 { last.get((j - 1) as usize) } else { None };
            let aft = last.get(j as usize);

            match (bef, aft) {
                (None, Some(x)) => inner.push(*x),
                (Some(x), None) => inner.push(*x),
                (Some(x), Some(y)) => inner.push(*x + *y),
                (None, None) => ()

            }
        }
        inner
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 { return Vec::new(); }

        let mut result: Vec<Vec<u32>> = vec![vec![1]];

        for i in 2..self.row_count + 1 {
            let last = result.get(result.len() - 1).unwrap().clone();
            result.push(self.add_numbers(last, i));
        }
        result
    }
}
