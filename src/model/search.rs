use super::{Sigil, TraitSet, Wrightstone};

#[derive(Debug, PartialEq)]
pub struct SearchQuery {
    pub desired_traits: TraitSet,
    pub sigil_slots: u8,
}

#[derive(Debug, Clone)]
pub struct SearchPool {
    pub sigils: Vec<Sigil>,
    pub wrightstones: Vec<Wrightstone>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub sigils: Vec<Sigil>,
    pub wrightstone: Option<Wrightstone>,
}

impl SearchResult {
    pub fn traits(&self) -> TraitSet {
        let mut traits = TraitSet::new();

        for sigil in &self.sigils {
            traits.add(sigil.trait1);
            if let Some(t2) = sigil.trait2 {
                traits.add(t2);
            }
        }

        if let Some(stone) = &self.wrightstone {
            traits.add(stone.trait1);
            if let Some(t2) = stone.trait2 {
                traits.add(t2);
            }
            if let Some(t3) = stone.trait3 {
                traits.add(t3);
            }
        }

        traits
    }
}
