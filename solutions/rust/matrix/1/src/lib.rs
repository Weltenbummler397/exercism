use std::collections::HashMap;
pub struct Matrix {
    // Implement your Matrix struct
    map: HashMap<(usize, usize), u32>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, num_str) in line.split_whitespace().enumerate() {
                if let Ok(num) = num_str.parse::<u32>() {
                    map.insert((i + 1, j + 1), num);
                }
            }
        }
        Matrix { map }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        let mut row: Vec<(usize, u32)> = self
            .map
            .iter()
            .filter(|&(&key, _)| key.0 == row_no)
            .map(|(&(row, col), &value)| (col, value))
            .collect();
        row.sort_by_key(|&(col, _)| col);
        let row_values: Vec<u32> = row.into_iter().map(|(_, value)| value).collect();
        if row_values.is_empty() {
            None
        } else {
            Some(row_values)
        }
    }


    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let mut col: Vec<(usize, u32)> = self
            .map
            .iter()
            .filter(|&(&key, _)| key.1 == col_no)
            .map(|(&(row, col), &value)| (row, value))
            .collect();
        col.sort_by_key(|&(row, _)| row);
        let col_values: Vec<u32> = col.into_iter().map(|(_, value)| value).collect();
        if col_values.is_empty() {
            None
        } else {
            Some(col_values)
        }
    }
}
