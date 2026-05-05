pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num.to_string()
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect();
    let mut num_sum = 0;
    for i in &digits {
        num_sum += i.pow(digits.len() as u32);
    }
    num == num_sum
}
