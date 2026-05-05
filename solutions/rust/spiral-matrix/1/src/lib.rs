use std::cmp::min;
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![];
    let mut count = 1;
    if size == 0 {
        return vec![];
    }
    for i in 0..size {
        let mut row = vec![];
        for j in 0..size {
            let k = min(min(i, j), min(size-1-i, size-1-j));
            let m = size - 2*k;
            let start = 1 + 4 * k * (size-k); 
            let mut offset = 0;
            if i == k {
                offset = j - k;
            } else if j == size - k - 1 {
                offset = (m-1) + (i-k);
            } else if i == size - k -1 {
                offset = 2 * (m-1) + (size - k - 1 -j );
            } else {
                offset = 3 * (m-1) + (size-k-1-i);
            }
            row.push(start + offset);
        }
        result.push(row);
    }
    result
}
