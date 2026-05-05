#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if let Some(c) = string_digits.chars().find(|c| !c.is_ascii_digit()){
        return Err(Error::InvalidDigit(c));
    }
    let digits: Vec<usize> = string_digits.chars()
    .map(|c| c.to_digit(10).expect("Not a digit!") as usize)
    .collect();

    let length = digits.len();

    if span > length {
        return Err(Error::SpanTooLong)
    }
    let mut all_series = Vec::new();
    
    for i in 0..=(length-span){
        let mut mult = 1;
        for a in 0..span {
            mult *= digits[i+a];
        }
        all_series.push(mult)  
    }
    let max = all_series.iter().max().copied().unwrap_or(0);
    Ok(max as u64)
}
