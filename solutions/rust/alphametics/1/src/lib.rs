use std::collections::{HashMap, HashSet};

/// Versucht, ein Alphametics-Rätsel zu lösen.
/// Nur mit der Standardbibliothek, Backtracking-Ansatz und robustem Parsing für Exercism-Input.
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Robust für Exercism-Input: "SEND + MORE == MONEY" oder mehrzeilig
    let input = input.replace("==", "=");
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() != 2 {
        return None;
    }
    let left = parts[0];
    let right = parts[1];
    let operand_words: Vec<&str> = left
        .split('+')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let result_word = right.trim();
    if operand_words.is_empty() || result_word.is_empty() {
        return None;
    }

    // Collect all unique letters (in order of first appearance)
    let mut unique_letters = Vec::new();
    let mut seen = HashSet::new();
    for word in operand_words.iter().chain(std::iter::once(&result_word)) {
        for c in word.chars() {
            if c.is_ascii_alphabetic() && seen.insert(c) {
                unique_letters.push(c);
            }
        }
    }
    if unique_letters.len() > 10 {
        return None;
    }

    // Finde führende Buchstaben
    let mut leading_letters = HashSet::new();
    for word in operand_words.iter().chain(std::iter::once(&result_word)) {
        if let Some(c) = word.chars().next() {
            leading_letters.insert(c);
        }
    }

    // Hilfsfunktion: Wort zu Zahl
    fn word_to_number(word: &str, map: &HashMap<char, u8>) -> Option<u64> {
        let mut n = 0u64;
        for c in word.chars() {
            let d = *map.get(&c)? as u64;
            n = n * 10 + d;
        }
        Some(n)
    }

    // Backtracking
    fn backtrack(
        idx: usize,
        unique_letters: &[char],
        used: &mut [bool; 10],
        map: &mut HashMap<char, u8>,
        leading_letters: &HashSet<char>,
        operand_words: &[&str],
        result_word: &str,
    ) -> bool {
        if idx == unique_letters.len() {
            let operand_vals: Option<Vec<u64>> = operand_words.iter().map(|w| word_to_number(w, map)).collect();
            let result_val: Option<u64> = word_to_number(result_word, map);
            if let (Some(ops), Some(res)) = (operand_vals, result_val) {
                let sum: u64 = ops.iter().sum();
                return sum == res;
            }
            return false;
        }
        let c = unique_letters[idx];
        for d in 0u8..=9 {
            if used[d as usize] {
                continue;
            }
            if d == 0 && leading_letters.contains(&c) {
                continue;
            }
            used[d as usize] = true;
            map.insert(c, d);
            if backtrack(idx + 1, unique_letters, used, map, leading_letters, operand_words, result_word) {
                return true;
            }
            used[d as usize] = false;
            map.remove(&c);
        }
        false
    }

    let mut used = [false; 10];
    let mut map = HashMap::new();
    if backtrack(
        0,
        &unique_letters,
        &mut used,
        &mut map,
        &leading_letters,
        &operand_words,
        result_word,
    ) {
        return Some(map);
    }
    None
}
