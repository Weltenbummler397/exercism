pub fn number(user_number: &str) -> Option<String> {
    let mut digits: Vec<_> = user_number.chars().filter(|c| c.is_ascii_digit()).rev().collect();
    if digits.len() != 10 && digits.len() != 11 {
        return None;
    }
    if let Some(n) = digits.get(9) {
    if matches!(n, '1' | '0') {
        return None
    }}
    if let Some(nn) = digits.get(6) {
    if matches!(nn, '1' | '0') {
        return None
    }}

    if digits.len() == 11{
        let f = digits.remove(10);
        if f != '1'{
            return None
    }}
    let s: String = digits.into_iter().rev().collect();
    Some(s)
}


