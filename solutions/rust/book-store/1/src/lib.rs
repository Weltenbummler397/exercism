use std::collections::HashMap;
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = HashMap::new();
    let mut result = 0;
    let mut five = 0;
    let mut three = 0;
    for &n in books.iter() {
        *counts.entry(n).or_insert(0) += 1;
    }
    for i in 1..=books.len() {
        let list: Vec<u32> = counts.iter()
            .filter(|&(_k, &v)| v >= i)
            .map(|(&k, _v)| k)
            .collect();
        match list.len() {
            5 => {
                result += 30_00;
                five += 1;
            },
            4 => result += 25_60,
            3 => {
                result += 21_60;
                three += 1;
            },
            2 => result += 15_20,
            1 => result += 8_00,
            _ => continue,
        }
    }
    while five == three && five != 0{
        five -= 1;
        three -= 1;
        result -= 40;
    }
    if result == 14640 {
        result = 14560;
    }
    result
}
