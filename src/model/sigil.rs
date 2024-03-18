use std::fmt::Display;

use super::traits::Trait;

#[derive(Debug, Clone, PartialEq)]
pub struct Sigil {
    pub trait1: Trait,
    pub trait2: Option<Trait>,
}

impl Sigil {
    pub fn new_single(trait1: Trait) -> Self {
        Sigil {
            trait1,
            trait2: None,
        }
    }
}

impl Display for Sigil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.trait1)?;

        if let Some(t2) = self.trait2 {
            write!(f, " + {t2}")?;
        }

        write!(f, "]")
    }
}
