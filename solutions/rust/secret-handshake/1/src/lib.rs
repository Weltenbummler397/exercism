pub fn actions(n: u8) -> Vec<&'static str> {
    let actions_vec = vec!["wink", "double blink", "close your eyes", "jump"];
    let mut result = Vec::new();
    for (i, &action) in actions_vec.iter().enumerate() {
        if n & (1 << i) != 0 {
            result.push(action);
        }
    }
    // If the fifth (reverse) bit is set, reverse the result
    if n & 0b10000 != 0 {
        result.reverse();
    }
    result
}