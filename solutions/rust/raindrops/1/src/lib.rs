pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }

    // Wenn der String leer ist, wurde keine der Bedingungen erfüllt.
    // In diesem Fall geben wir die Zahl selbst als String zurück.
    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}