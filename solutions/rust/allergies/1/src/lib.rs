pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_value = *allergen as u32;
        (self.score & allergen_value) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = Vec::new();
        for i in 0..=8 {
            let mask = 1 << i;
            if self.score & mask != 0 {
                match mask {
                    1 => result.push(Allergen::Eggs),
                    2 => result.push(Allergen::Peanuts),
                    4 => result.push(Allergen::Shellfish),
                    8 => result.push(Allergen::Strawberries),
                    16 => result.push(Allergen::Tomatoes),
                    32 => result.push(Allergen::Chocolate),
                    64 => result.push(Allergen::Pollen),
                    128 => result.push(Allergen::Cats),
                    _ => (),
                }
            }
        }
        result
    }
}
