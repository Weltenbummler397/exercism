#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = vec![0; (max_weight + 1) as usize];
    for item in items {
        for w in (item.weight..=max_weight).rev() {
            let w_usize = w as usize;
            let prev_w_usize = (w - item.weight) as usize;
            dp[w_usize] = dp[w_usize].max(dp[prev_w_usize] + item.value);
        }
    }
    dp[max_weight as usize]
}