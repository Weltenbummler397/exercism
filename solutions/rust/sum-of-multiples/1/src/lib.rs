pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    for i in factors {
        let mut count = 0;
        if *i != 0 {
            loop{
                count += 1;
                if i*count >= limit {
                    count = 0;
                    break;
                }
                multiples.push(i*count);
                }
            }
        }
    multiples.sort();
    multiples.dedup();
    for i in multiples {
        sum += i;
    }
    sum
}
