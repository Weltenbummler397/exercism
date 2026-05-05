pub fn series(digits: &str, len: usize) -> Vec<String> {
    let zeichen_vec: Vec<char> = digits.chars().collect();
    let len_vec = zeichen_vec.len();
    let mut result: Vec<String> = Vec::new();

    if len_vec < len {
        return result;
    }
    
    for i in 0..=(len_vec - len) {
        let mut storage = String::new();
            for j in 0..len {
            storage.push(zeichen_vec[i+j]);
            }
            result.push(storage);

    }
    result
}
