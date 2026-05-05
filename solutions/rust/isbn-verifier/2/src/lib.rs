/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let check = isbn.chars().filter(|c| (('a'..='z').contains(c) || ('A'..='Z').contains(c)) && *c != 'X').collect::<Vec<char>>();
        if check.len() != 0 {
        return false;
    }
    let mut digits: Vec<char> = isbn.chars().filter(|c| ('0'..='9').contains(c) || *c == 'X').collect::<Vec<char>>();
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
