use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
	for (key, value) in h {
		for i in value {
			for lower in i.to_lowercase() {
				result.insert(lower, *key);
			}
		}
	}
	result
}
