pub fn annotate(garden: &[&str]) -> Vec<String> {
    let zeilen = garden.len();
    if zeilen == 0 {return vec![];}
    let spalten = garden[0].len();

    let mut ergebnis = vec![vec![' '; spalten]; zeilen];

    for r in 0..zeilen {
        for c in 0..spalten {
            if garden[r].chars().nth(c) == Some('*') {
                ergebnis[r][c] = '*';

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {continue;}

                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
    
                        if nr>= 0 && nr < zeilen as isize && nc >=0 && nc<spalten as isize {
                            let nr = nr as usize;
                            let nc = nc as usize;
    
                            if ergebnis[nr][nc] != '*' {
                                let aktueller_wert = ergebnis[nr][nc].to_digit(10).unwrap_or(0);
                                ergebnis[nr][nc] = std::char::from_digit(aktueller_wert + 1, 10).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    let mut final_vector = Vec::new();
    for r in 0..zeilen {
        let zeilen_string: String = ergebnis[r].iter().collect();
        final_vector.push(zeilen_string);
    }

    final_vector
}
