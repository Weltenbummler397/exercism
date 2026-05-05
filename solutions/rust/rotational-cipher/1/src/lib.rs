pub fn rotate(input: &str, key: u8) -> String {
    let mut output = String::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for chars in input.chars() {
        match chars {
            x @ 'a'..='z' => {
                let mut index = alphabet.find(x).unwrap() + key as usize;
                while index >= 26 {
                    index -= 26;
                }
                output.push(alphabet.chars().nth(index).unwrap());
            }
            y @ 'A'..='Z' => {
                let mut index = alphabet.to_uppercase().find(y).unwrap() + key as usize;
                while index >= 26 {
                    index -= 26;
                }
                output.push(alphabet.to_uppercase().chars().nth(index).unwrap());
            }
            ' ' | '0'..='9' | _ => output.push(chars),
            _ => (),
        }
    }
    output
}
