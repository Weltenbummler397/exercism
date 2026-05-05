pub fn abbreviate(phrase: &str) -> String {
    let gefiltert: String = phrase.chars().map(|c| if c.is_alphabetic() || c=='\'' { c } else { ' ' }).collect();
    let teile: Vec<&str> = gefiltert.split_whitespace().collect();
    let mut result = String::new();
    
    for i in teile {
        let up = i.to_uppercase();
        let first = up.chars().next().unwrap();
        result.push(first);
        
        if !i.chars().all(|c| c.is_uppercase()) {
            let len = i.chars().count();
            let middle_upper: Option<char> = i.chars()
            .enumerate()
            .filter(|(j, _)| *j > 0 && *j < len - 1)
            .find_map(|(_, c)| if c.is_uppercase() { Some(c) } else { None });
            if middle_upper.is_some() {
                let middle_upper_c = middle_upper.unwrap();    
                result.push(middle_upper_c);
            }
        }
    }
    result
}