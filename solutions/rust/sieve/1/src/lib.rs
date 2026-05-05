pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let digits: Vec<_> = (2..=upper_bound).collect();
    let mut result: Vec<u64> = Vec::new();
    for digit in digits {
        match digit {
            2 => result.push(2),
            val if val % 2 == 0 => (),
            _ => {if !result.iter().any(|&i| digit % i == 0) {
                    result.push(digit);
                    }
                }
            }
        }
    result
}
