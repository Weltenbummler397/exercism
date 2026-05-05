pub fn check(candidate: &str) -> bool {
    let candidate = candidate.to_lowercase();
    let mut chars: Vec<char> = candidate.chars().filter(|c| {*c != ' ' && *c != '-'}).collect();
    chars.sort();
    for i in 1..chars.len() {
        if chars[i] == chars[i-1] {
            return false;
        }
    }
    true
}
