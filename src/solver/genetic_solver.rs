use rand::prelude::*;

use crate::model::{SearchPool, SearchQuery, SearchResult};

use super::util::filter_pool;
use super::Solver;

/// a solver using a genetic algorithm
pub struct GeneticSolver {
    pub population_size: usize,
    pub generations: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
}

impl Solver for GeneticSolver {
    fn search(&self, pool: SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
        let filtered_pool = filter_pool(pool, query);
        let population = Population::new(self.population_size, &filtered_pool, query);
        dbg!(&population);
        todo!()
    }
}

#[derive(Debug)]
struct Population {
    genomes: Vec<SearchResult>,
}

impl Population {
    pub fn new(size: usize, pool: &SearchPool, query: &SearchQuery) -> Self {
        Population {
            genomes: (0..size).map(|_| random_combination(pool, query)).collect(),
        }
    }
}

fn random_combination(pool: &SearchPool, query: &SearchQuery) -> SearchResult {
    let mut rng = rand::thread_rng();

    let sigils = pool
        .sigils
        .choose_multiple(&mut rng, query.sigil_slots as usize)
        .cloned()
        .collect();
    let wrightstone = pool.wrightstones.choose(&mut rng).cloned();

    SearchResult {
        sigils,
        wrightstone,
    }
}
