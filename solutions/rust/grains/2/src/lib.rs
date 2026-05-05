pub fn square(s: u32) -> u64 {
    let mut grains = 1;
    for n in 1..=64 {
        if s == n{
            println!("Die Anzahl an Körnern auf Feld {} ist: {}", s, grains);
            break;
        } else {
            grains *= 2;
        }
    }
    grains
}

pub fn total() -> u64 {
    let mut grains_total = 0;
        for n in 0..64 {
        let grains_on_square = 1u64 << n;
        grains_total += grains_on_square;
    }
    grains_total
}
