use rand::prelude::*;

use crate::model::{SearchPool, SearchQuery, SearchResult};

use super::util::{filter_pool, is_valid_result};
use super::Solver;

/// a solver using a genetic algorithm
pub struct GeneticSolver {
    pub population_size: usize,
    pub generations: usize,
    pub tournament_size: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
}

impl Solver for GeneticSolver {
    fn search(&self, pool: SearchPool, query: &SearchQuery) -> Vec<SearchResult> {
        let filtered_pool = filter_pool(pool, query);
        let mut population = Population::new(
            self.population_size,
            self.tournament_size,
            self.crossover_rate,
            self.mutation_rate,
            &filtered_pool,
            query,
        );
        dbg!(&population);

        for i in 0..self.generations {
            println!("running generation {i}");
            population.next_generation();
        }

        population.valid_results(query)
    }
}

#[derive(Debug)]
struct Population<'s> {
    genomes: Vec<SearchResult>,
    tournament_size: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    query: &'s SearchQuery,
}

impl<'s> Population<'s> {
    fn new(
        size: usize,
        tournament_size: usize,
        crossover_rate: f64,
        mutation_rate: f64,
        pool: &SearchPool,
        query: &'s SearchQuery,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Population {
            genomes: (0..size)
                .map(|_| random_combination(pool, query, &mut rng))
                .collect(),
            tournament_size,
            crossover_rate,
            mutation_rate,
            query,
        }
    }

    fn valid_results(self, query: &SearchQuery) -> Vec<SearchResult> {
        self.genomes
            .into_iter()
            .filter(|res| is_valid_result(res, query))
            .collect()
    }

    fn next_generation(&mut self) {
        let mut rng = rand::thread_rng();
        let parents = self.select(&mut rng);
        let mut children = self.crossover(&parents, &mut rng);
        self.mutate(&mut children, &mut rng);
        self.genomes = children;
    }

    fn select(&self, rng: &mut ThreadRng) -> Vec<&SearchResult> {
        let len = self.genomes.len();
        let mut parents = Vec::with_capacity(len);

        loop {
            let best = self
                .genomes
                .choose_multiple(rng, self.tournament_size)
                .max_by_key(|g| fitness(&g, self.query))
                .unwrap();
            parents.push(best);

            if parents.len() == len {
                break;
            }
        }

        parents
    }

    fn crossover(&self, parents: &[&SearchResult], rng: &mut ThreadRng) -> Vec<SearchResult> {
        let mut children = Vec::with_capacity(parents.len());

        for parents in parents.chunks_exact(2) {
            let parent1 = parents[0];
            let parent2 = parents[1];
            assert_eq!(parent1.sigils.len(), parent2.sigils.len());
            let len = parent1.sigils.len();

            // TODO: crossover wrightstones
            if rng.gen_bool(self.crossover_rate) {
                let point = rng.gen_range(0..len);
                let p1_1 = &parent1.sigils[..point];
                let p1_2 = &parent1.sigils[point..];
                let p2_1 = &parent2.sigils[..point];
                let p2_2 = &parent2.sigils[point..];

                let child1_sigils = [p1_1, p2_2].into_iter().flatten().cloned().collect();
                let child2_sigils = [p2_1, p1_2].into_iter().flatten().cloned().collect();

                children.push(SearchResult {
                    sigils: child1_sigils,
                    wrightstone: parent1.wrightstone.clone(),
                });
                children.push(SearchResult {
                    sigils: child2_sigils,
                    wrightstone: parent2.wrightstone.clone(),
                });
            } else {
                children.push(parent1.clone());
                children.push(parent2.clone());
            }
        }

        children
    }

    fn mutate(&self, children: &mut Vec<SearchResult>, rng: &mut ThreadRng) -> Vec<SearchResult> {
        // TODO: find a way to randomly select unused sigils from the pool
        todo!()
    }
}

fn random_combination(pool: &SearchPool, query: &SearchQuery, rng: &mut ThreadRng) -> SearchResult {
    let sigils = pool
        .sigils
        .choose_multiple(rng, query.sigil_slots as usize)
        .cloned()
        .collect();
    let wrightstone = pool.wrightstones.choose(rng).cloned();

    SearchResult {
        sigils,
        wrightstone,
    }
}

fn fitness(result: &SearchResult, query: &SearchQuery) -> i32 {
    // TODO: create an algorithm that can quantify how well a search result fulfills the query
    // must also give a positive score to results that dont meet the requirements
    // so that they can be sorted
    todo!()
}
