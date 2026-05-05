pub fn rotate(input: &str, key: u8) -> String {
    let mut output = String::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for chars in input.chars() {
        match chars {
            'a'..='z' | 'A'..='Z' => {
                let mut index = alphabet.find(chars.to_ascii_lowercase()).unwrap() + key as usize;
                while index >= 26 {
                    index -= 26;
                }
                if chars.is_uppercase() {
                    output.push(alphabet.chars().nth(index).unwrap().to_ascii_uppercase());
                } else {
                    output.push(alphabet.chars().nth(index).unwrap());
                }
            }
            ' ' | '0'..='9' | _ => output.push(chars),
        }
    }
    output
}
