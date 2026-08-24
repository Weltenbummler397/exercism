use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut char_hash = HashSet::new();
    
    // 1. Ausgangswort in Kleinbuchstaben umwandeln
    let word_lower = word.to_lowercase();
    
    // 2. Häufigkeit der Buchstaben des Ausgangsworts zählen
    let mut word_counts = HashMap::new();
    for c in word_lower.chars() {
        *word_counts.entry(c).or_insert(0) += 1;
    }

    for &kandidat in possible_anagrams {
        let kandidat_lower = kandidat.to_lowercase();

        // Ein Wort ist kein Anagramm von sich selbst
        if kandidat_lower == word_lower {
            continue;
        }

        // Schnelle Vorabprüfung: Wenn die Zeichenanzahl (Unicode-Chars) ungleich ist, abbrechen
        if kandidat_lower.chars().count() != word_lower.chars().count() {
            continue;
        }

        // 3. Häufigkeit der Buchstaben des Kandidaten zählen
        let mut kandidat_counts = HashMap::new();
        for c in kandidat_lower.chars() {
            *kandidat_counts.entry(c).or_insert(0) += 1;
        }

        // 4. Nur wenn die Maps exakt identisch sind, ist es ein echtes Anagramm
        if word_counts == kandidat_counts {
            char_hash.insert(kandidat);
        }
    }

    char_hash
}
