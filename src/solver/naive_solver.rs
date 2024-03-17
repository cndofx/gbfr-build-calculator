use itertools::Itertools;

use crate::model::{SearchPool, SearchQuery, SearchResult};

use super::util::{filter_pool, is_valid_result};
use super::Solver;

/// a naive solver using a brute force algorithm
pub struct NaiveSolver;

impl Solver for NaiveSolver {
    fn search(&self, pool: SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
        let filtered_pool = filter_pool(pool, query);
        let combinations = all_combinations(filtered_pool, query);
        println!("Checking {} combinations...", combinations.len());
        let valid = combinations
            .into_iter()
            .filter(|c| is_valid_result(c, query))
            .collect();
        valid
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
