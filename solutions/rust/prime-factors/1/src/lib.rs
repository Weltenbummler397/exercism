pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut divisor = 2;
    let mut prime_factors: Vec<u64> = Vec::new();
    
    if num < 2 {
        return prime_factors;
    }
    
    while num > 1 {
        while num % divisor == 0 {
            prime_factors.push(divisor);
            num /= divisor;
        }
        divisor += 1;
    }
    prime_factors
}
