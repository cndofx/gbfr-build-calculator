use std::collections::HashMap;

use itertools::Itertools;

use crate::model::{SearchPool, SearchQuery, SearchResult, Trait};

use super::Solver;

pub struct BasicSolver;

impl Solver for BasicSolver {
    fn search(&self, pool: &SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
        let combinations = combinations(pool, query);
        let filtered = combinations
            .into_iter()
            .filter(|c| is_valid_combination(c, query))
            .collect();
        filtered
    }
}

fn combinations(pool: &SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
    let num_sigils = std::cmp::min(query.sigil_slots as usize, pool.sigils.len());
    let combinations = pool
        .clone()
        .sigils
        .into_iter()
        .combinations(num_sigils)
        .map(|sigils| SearchResult {
            sigils,
            wrightstone: None,
        })
        .collect();
    combinations
}

fn is_valid_combination(combo: &SearchResult, query: &SearchQuery) -> bool {
    let mut traits: HashMap<Trait, u8> = HashMap::new();
    for sigil in combo.sigils.iter() {
        traits
            .entry(sigil.trait1)
            .and_modify(|x| *x += sigil.level)
            .or_insert(sigil.level);
        if let Some(trait2) = sigil.trait2 {
            traits
                .entry(trait2)
                .and_modify(|x| *x += sigil.level)
                .or_insert(sigil.level);
        }
    }

    query
        .desired_traits
        .iter()
        .all(|(t, desired_level)| traits.get(t).is_some_and(|n| n >= desired_level))
}
