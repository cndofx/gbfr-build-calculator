use std::fmt::Display;

use super::Trait;

#[derive(Debug, Clone, PartialEq)]
pub struct Wrightstone {
    pub trait1: Trait,
    pub trait2: Option<Trait>,
    pub trait3: Option<Trait>,
}

impl Display for Wrightstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.trait1)?;

        if let Some(t2) = self.trait2 {
            write!(f, " + {t2}")?;
        }

        if let Some(t3) = self.trait3 {
            write!(f, " + {t3}")?;
        }

        write!(f, "]")
    }
}
