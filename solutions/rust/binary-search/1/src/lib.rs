pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut list: Vec<i32> = array.to_vec();
    list.sort();

    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if list[mid] == key {
            return array.iter().position(|&x| x == key);
        } else if list[mid] < key {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}