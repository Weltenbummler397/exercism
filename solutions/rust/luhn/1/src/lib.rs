/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !c.is_ascii_digit() && !c.is_whitespace()) {
        return false;
    }
    let mut digits: Vec<u32> = code.chars().filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits.len() <= 1 {
        return false;
    }

    for i in (0..digits.len()).rev().skip(1).step_by(2) {
        let mut mult = digits[i] *2;
        if mult > 9 {
            mult -= 9;
        }
        digits[i] = mult;
    }
    if digits.iter().sum::<u32>() % 10 != 0 {
        return false;
    } else {
    true
    }
}
