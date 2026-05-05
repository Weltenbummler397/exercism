use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    number: String,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.number)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut number = num;
        let mut result = String::new();

        let roman_numerals = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        while number > 0 {
            match number {
                n if n >= 1000 => {
                    result.push_str("M");
                    number -= 1000;
                }
                n if n >= 900 => {
                    result.push_str("CM");
                    number -= 900;
                }
                n if n >= 500 => {
                    result.push_str("D");
                    number -= 500;
                }
                n if n >= 400 => {
                    result.push_str("CD");
                    number -= 400;
                }
                n if n >= 100 => {
                    result.push_str("C");
                    number -= 100;
                }
                n if n >= 90 => {
                    result.push_str("XC");
                    number -= 90;
                }
                n if n >= 50 => {
                    result.push_str("L");
                    number -= 50;
                }
                n if n >= 40 => {
                    result.push_str("XL");
                    number -= 40;
                }
                n if n >= 10 => {
                    result.push_str("X");
                    number -= 10;
                }
                n if n >= 9 => {
                    result.push_str("IX");
                    number -= 9;
                }
                n if n >= 5 => {
                    result.push_str("V");
                    number -= 5;
                }
                n if n >= 4 => {
                    result.push_str("IV");
                    number -= 4;
                }
                n if n >= 1 => {
                    result.push_str("I");
                    number -= 1;
                }
                _ => {}
            }
        }
        Roman { number: result }
    }
}
