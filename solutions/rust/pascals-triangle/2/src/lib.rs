pub struct PascalsTriangle{
     generate_rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows:Vec<Vec<u32>> = Vec::new();
        for i in 0..row_count {
            let mut current = Vec::new();
            for j in 0..=i {
                if j==0 || j==i {
                    current.push(1);
                } else {
                    let summe = rows[(i - 1) as usize][(j - 1) as usize] + rows[(i - 1) as usize][j as usize];
                current.push(summe);
                }
            }
            rows.push(current);
        }
        Self { generate_rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.generate_rows.clone()
    }
}
