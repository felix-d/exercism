pub struct PascalsTriangle {
  rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
      PascalsTriangle {
        rows: PascalsTriangle::build_rows(row_count)
      }
    }

    fn build_rows(row_count: u32) -> Vec<Vec<u32>> {
      (0..row_count).map(PascalsTriangle::build_row).collect()
    }

    fn build_row(row_number: u32) -> Vec<u32> {
      (0..row_number).fold(vec![1], |mut row, column_index| {
        row.push(row[column_index as usize] * (row_number - column_index) / (column_index + 1));
        row
      })
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
      self.rows.clone()
    }
}
