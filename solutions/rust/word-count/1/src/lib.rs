use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let cleaned: String = words
        .chars()
        .map(|c| if ",.!?;:\"".contains(c) { ' ' } else { c })
        .collect();

    let mut counts = HashMap::new();
    for word in cleaned.split_whitespace() {
        let word = word.trim_matches('\'');
        let word = word.to_lowercase();
        if !word.is_empty() && word.chars().any(|c| c.is_alphanumeric()) {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    counts
}

