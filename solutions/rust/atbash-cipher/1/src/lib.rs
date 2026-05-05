/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut plain = plain.to_lowercase();
    let mut encoded: Vec<char> = Vec::new();
    let mut result = String::new();
    for c in plain.chars() {
        match c {
            'a' => encoded.push('z'),
            'b' => encoded.push('y'),
            'c' => encoded.push('x'),
            'd' => encoded.push('w'),
            'e' => encoded.push('v'),
            'f' => encoded.push('u'),
            'g' => encoded.push('t'),
            'h' => encoded.push('s'),
            'i' => encoded.push('r'),
            'j' => encoded.push('q'),
            'k' => encoded.push('p'),
            'l' => encoded.push('o'),
            'm' => encoded.push('n'),
            'n' => encoded.push('m'),
            'o' => encoded.push('l'),
            'p' => encoded.push('k'),
            'q' => encoded.push('j'),
            'r' => encoded.push('i'),
            's' => encoded.push('h'),
            't' => encoded.push('g'),
            'u' => encoded.push('f'),
            'v' => encoded.push('e'),
            'w' => encoded.push('d'),
            'x' => encoded.push('c'),
            'y' => encoded.push('b'),
            'z' => encoded.push('a'),
            '0'..='9' => encoded.push(c),
            _ => continue, 
        }
    }
    for (i, c) in encoded.iter().enumerate() {
        if i > 0 && i % 5 == 0 {
            result.push(' ');
        }
        result.push(*c);
    }
    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result = String::new();
    for c in cipher.chars() {
        match c {
            'a' => result.push('z'),
            'b' => result.push('y'),
            'c' => result.push('x'),
            'd' => result.push('w'),
            'e' => result.push('v'),
            'f' => result.push('u'),
            'g' => result.push('t'),
            'h' => result.push('s'),
            'i' => result.push('r'),
            'j' => result.push('q'),
            'k' => result.push('p'),
            'l' => result.push('o'),
            'm' => result.push('n'),
            'n' => result.push('m'),
            'o' => result.push('l'),
            'p' => result.push('k'),
            'q' => result.push('j'),
            'r' => result.push('i'),
            's' => result.push('h'),
            't' => result.push('g'),
            'u' => result.push('f'),
            'v' => result.push('e'),
            'w' => result.push('d'),
            'x' => result.push('c'),
            'y' => result.push('b'),
            'z' => result.push('a'),
            '0'..='9' => result.push(c),
            _ => continue, 
        }
    }
    result
}
