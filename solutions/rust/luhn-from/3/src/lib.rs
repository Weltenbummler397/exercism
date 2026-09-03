pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.code.chars().any(|c| !c.is_ascii_digit() && !c.is_whitespace()) {
            return false;
        }
        let mut digits: Vec<u32> = self.code.chars().filter(|c| c.is_ascii_digit())
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
        digits.iter().sum::<u32>() % 10 == 0 
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn{code: input.to_string()}
    }
}
