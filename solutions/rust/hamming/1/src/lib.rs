/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut s1_vec: Vec<_> = s1.chars().collect();
    let mut s2_vec: Vec<_> = s2.chars().collect();

    let mut count = 0;
    
    for _ in 0..=s1.len() {
        if s1_vec.pop() != s2_vec.pop() {
            count += 1;        
        }
    }
    Some(count)
}
