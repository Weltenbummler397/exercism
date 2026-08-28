pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let n = input[0].len();
    let mut row_max = Vec::new();
    let mut column_min = vec![u64::MAX; n];
    for i in input {
        if let Some(max_val) = i.iter().max().copied() {
        row_max.push(max_val);
        }
        for j in 0..n {
            if i[j] < column_min[j] {
                column_min[j] = i[j];
            }
        }
    }
    let mut result: Vec<(usize, usize)> = Vec::new(); 
    let m = input.len();
    for i in 0..m {
        for j in 0..n {
            let wert = input[i][j];
            if wert == row_max[i] && wert == column_min[j] {
                result.push((i,j));
            }
        }
    }
    result
}
