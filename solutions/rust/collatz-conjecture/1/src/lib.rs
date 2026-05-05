pub fn collatz(n: u64) -> Option<u64> {
    let mut zahl = n;
    let mut count = 0;

    if zahl == 0 {
        return None;
    }
    while zahl != 1 && zahl > 0{
        if zahl % 2 == 0 {
            zahl /= 2;
        } else {
            zahl = (zahl*3)+1 
        }
        count += 1
    }
        Some(count)
}
