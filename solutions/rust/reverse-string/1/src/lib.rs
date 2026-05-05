pub fn reverse(input: &str) -> String {
    let input_trim = input.trim();
    input.trim().chars().rev().collect()
}
