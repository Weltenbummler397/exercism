/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let word_vec: Vec<_> = word.chars().map(|c| c.to_ascii_uppercase()).collect();
    let mut result = 0;

    for i in word_vec {
        match i {
            'Q'|'Z' => result += 10,
            'J'|'X' => result += 8,
            'K' => result += 5,
            'F'|'H'|'V'|'W'|'Y' => result += 4,
            'B'|'C'|'M'|'P' => result += 3,
            'D'|'G' => result += 2,
            'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' => result += 1,
            _ => {},
        }
    }
    result
}
