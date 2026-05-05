/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let check = isbn.chars().filter(|c| (c.is_ascii_lowercase() || c.is_ascii_uppercase()) && *c != 'X').collect::<Vec<char>>();
        if !check.is_empty()  {
        return false;
    }
    let digits: Vec<char> = isbn.chars().filter(|c| c.is_ascii_digit() || *c == 'X').collect::<Vec<char>>();
    if digits.len() != 10 {
        return false;
    }
    let mut sum = 0;
    let mut counter = 10;
    for i in digits {
        if i == 'X' {
            sum += 10 * counter;
        } else {
            sum += i.to_digit(10).unwrap() * counter;
            counter -= 1;
        }
    }
    sum % 11 == 0
}
