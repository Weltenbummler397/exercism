/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut is_pangram = sentence.to_lowercase();
    is_pangram = is_pangram.chars().filter(|c| c.is_alphabetic()).collect();
    let mut letters = std::collections::HashSet::new();
    for c in is_pangram.chars() {
        letters.insert(c);
    }
    for x in 'a'..='z' {
        if letters.contains(&x) == false {
            return false;
        }
    }
    true
}
