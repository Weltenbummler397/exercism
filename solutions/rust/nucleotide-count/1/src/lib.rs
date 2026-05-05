    use std::collections::HashMap;

    pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
        if dna.contains(|c: char| !matches!(c, 'A' | 'C' | 'G' | 'T')) {
            return Err(dna.chars().find(|&c| !matches!(c, 'A' | 'C' | 'G' | 'T')).unwrap());
        } else if !matches!(nucleotide, 'A' | 'C' | 'G' | 'T') {
            return Err(nucleotide);
        }
        let slices = dna.chars().filter(|&c| c == nucleotide).count();
        Ok(slices)
    }

    pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
        if dna.contains(|c: char| !matches!(c, 'A' | 'C' | 'G' | 'T')) {
            return Err(dna.chars().find(|&c| !matches!(c, 'A' | 'C' | 'G' | 'T')).unwrap());
        }
        let nucleotides = ['A', 'C', 'G', 'T'];
        let mut counts = HashMap::new();
        for &nucleotide in &nucleotides {
            let count = dna.chars().filter(|&c| c == nucleotide).count();
            counts.insert(nucleotide, count);
        }
        Ok(counts)
    }
