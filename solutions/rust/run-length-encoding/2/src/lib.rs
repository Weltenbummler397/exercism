pub fn encode(source: &str) -> String {
    let mut count = 1;
    let mut result = String::new();
    for i in 0..source.len() {
        if let Some(first) = source.chars().nth(i) {
            if let Some(second) = source.chars().nth(i+1) {
                    if first == second {
                        count += 1;
                    } else {
                        if count > 1 {
                            result.push_str(&count.to_string());
                        }
                        result.push_str(&first.to_string());
                        count = 1; 
                }
            } else if let Some(last) = source.chars().last() {
                if count > 1 {
                    result.push_str(&count.to_string());
            }
            result.push_str(&last.to_string());
            }
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    // String zum Sammeln von Ziffern (z.B. "12A" -> number = "12")
    let mut number = String::new();
    // Ergebnis-String
    let mut result = String::new();
    // Über alle Zeichen im Quell-String iterieren
    for i in 0..source.len() {
        // Das aktuelle Zeichen holen (Option<char>)
        if let Some(c) = source.chars().nth(i) {
            // Wenn das Zeichen eine Ziffer ist, an number anhängen
            if c.is_ascii_digit() {
                number.push(c);
            } else {
                // Sonst: number als Zahl interpretieren (oder 1, falls leer)
                for _ in 0..number.parse::<usize>().unwrap_or(1) {
                    result.push(c);
                }
                // number zurücksetzen
                number.clear();
            }
        }
    }
    // Ergebnis zurückgeben
    result
}
