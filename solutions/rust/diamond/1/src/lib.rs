pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as i32;
    let g = 2*n+1;
    let mut diamanten_zeile = Vec::new();
    for y in 0..g {
        let mut aktuell = String::new();

        for x in 0..g {
            let abs_x = (n-x).abs();
            let abs_y = (n-y).abs();

            if abs_x+abs_y == n {
                let buchstabe = (b'A' + (n - abs_y) as u8) as char;
                aktuell.push(buchstabe);
            } else {
                aktuell += " ";
            }
        }
        diamanten_zeile.push(aktuell);
    }
    diamanten_zeile
}
