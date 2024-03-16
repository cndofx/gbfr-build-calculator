use std::collections::HashMap;

use itertools::Itertools;

use crate::model::{SearchPool, SearchQuery, SearchResult, Trait};

use super::Solver;

pub struct BasicSolver;

impl Solver for BasicSolver {
    fn search(&self, pool: SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
        let filtered_pool = filter_pool(pool, query);
        let combinations = all_combinations(filtered_pool, query);
        println!("Checking {} combinations...", combinations.len());
        let valid = combinations
            .into_iter()
            .filter(|c| is_valid_combination(c, query))
            .collect();
        valid
    }
}

fn filter_pool(pool: SearchPool, query: &SearchQuery) -> SearchPool {
    let SearchPool {
        mut sigils,
        mut wrightstones,
    } = pool;

    sigils = sigils
        .into_iter()
        .filter(|sigil| {
            let trait1 = Some(sigil.trait1);
            let trait2 = sigil.trait2;
            trait1.is_some_and(|(t, _)| is_desired_trait(t, query))
                || trait2.is_some_and(|(t, _)| is_desired_trait(t, query))
        })
        .collect();

    wrightstones = wrightstones
        .into_iter()
        .filter(|stone| {
            let trait1 = Some(stone.trait1);
            let trait2 = stone.trait2;
            let trait3 = stone.trait3;
            trait1.is_some_and(|(t, _)| is_desired_trait(t, query))
                || trait2.is_some_and(|(t, _)| is_desired_trait(t, query))
                || trait3.is_some_and(|(t, _)| is_desired_trait(t, query))
        })
        .collect();

    SearchPool {
        sigils,
        wrightstones,
    }
}

fn all_combinations(pool: SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
    let num_sigils = std::cmp::min(query.sigil_slots as usize, pool.sigils.len());

    let SearchPool {
        sigils,
        wrightstones,
    } = pool;

    let combinations = if wrightstones.len() > 0 {
        sigils
            .into_iter()
            .combinations(num_sigils)
            .cartesian_product(wrightstones.into_iter())
            .map(|(sigils, wrightstone)| SearchResult {
                sigils,
                wrightstone: Some(wrightstone),
            })
            .collect()
    } else {
        sigils
            .into_iter()
            .combinations(num_sigils)
            .map(|sigils| SearchResult {
                sigils,
                wrightstone: None,
            })
            .collect()
    };

    combinations
}

fn is_valid_combination(combo: &SearchResult, query: &SearchQuery) -> bool {
    let mut traits: HashMap<Trait, u8> = HashMap::new();
    for sigil in combo.sigils.iter() {
        let (t1, l1) = sigil.trait1;
        traits.entry(t1).and_modify(|x| *x += l1).or_insert(l1);
        if let Some(trait2) = sigil.trait2 {
            let (t2, l2) = trait2;
            traits.entry(t2).and_modify(|x| *x += l2).or_insert(l2);
        }
    }

    query
        .desired_traits
        .iter()
        .all(|(t, desired_level)| traits.get(t).is_some_and(|n| n >= desired_level))
}

fn is_desired_trait(t: Trait, query: &SearchQuery) -> bool {
    query.desired_traits.contains_key(&t)
}
