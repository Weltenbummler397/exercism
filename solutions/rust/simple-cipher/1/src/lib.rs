use rand::Rng;

fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}

fn stretch_key(key: &str, len: usize) -> String {
    key.chars().cycle().take(len).collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let key_stretch = stretch_key(key, s.len());
    let mut res = String::with_capacity(s.len());
    for (kc, sc) in key_stretch.bytes().zip(s.bytes()) {
        if sc < b'a' || sc > b'z' {
            return None;
        }
        let offset = ((kc - b'a') + (sc - b'a')) % 26;
        res.push((b'a' + offset) as char);
    }
    Some(res)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let key_stretch = stretch_key(key, s.len());
    let mut res = String::with_capacity(s.len());
    for (kc, sc) in key_stretch.bytes().zip(s.bytes()) {
        if sc < b'a' || sc > b'z' {
            return None;
        }
        let offset = (26 + (sc - b'a') - (kc - b'a')) % 26;
        res.push((b'a' + offset) as char);
    }
    Some(res)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key_len = 100.max(s.len());
    let mut rng = rand::thread_rng();
    let key: String = (0..key_len)
        .map(|_| (b'a' + rng.gen_range(0..26)) as char)
        .collect();

    // ** Use only the first s.len() chars for actual encoding, per test! **
    let encoded = encode(&key[..s.len()], s).unwrap();

    (key, encoded)
}