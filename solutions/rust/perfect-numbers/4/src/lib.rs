#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else if num == 1 {
        return Some(Classification::Deficient);
    }
    let mut digit = num/2;
    let mut aliquot = Vec::new();
    while digit > 1 {
        if num.is_multiple_of(digit) {
            aliquot.push(digit);
        }
        digit -= 1;
    }
    aliquot.push(1);
    let result: u64 = aliquot.into_iter().sum();
    if result == num {
        Some(Classification::Perfect)
    } else if result > num {
        Some(Classification::Abundant)
    } else if result < num {
        Some(Classification::Deficient)
    } else {
        None
    }
}
