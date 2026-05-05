#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some((idx, _)) = dna.chars().enumerate().find(|&(_, c)| !matches!(c, 'A'|'C'|'G'|'T'))
        {
            return Err(idx);
        }
        Ok(Dna { sequence: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        let sequence = self.sequence.chars().map(|c| match c {
            'A' => 'U',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => unreachable!(),
        }).collect();
        Rna { sequence }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some((idx, _)) = rna.chars().enumerate().find(|&(_, c)| !matches!(c, 'A'|'C'|'G'|'U'))
        {
            return Err(idx);
        }
        Ok(Rna { sequence: rna.to_string() })
    }
}
