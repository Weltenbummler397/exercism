pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    let mut sum_1 = 0;
    for i in 1..=n {
        sum_1 += i;
    }
    let sqrt = sum_1.pow(2);

    let mut sum = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    sqrt - sum
}
