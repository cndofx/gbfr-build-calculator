// inspired by https://github.com/TanukiSharp/MHArmory/blob/master/MHArmory.Search/Documentation/README.md

use crate::model::{SearchPool, SearchQuery, SearchResult};

mod genetic_solver;
mod naive_solver;
pub mod util;

pub use genetic_solver::GeneticSolver;
pub use naive_solver::NaiveSolver;

pub trait Solver {
    fn search(&self, pool: SearchPool, query: &SearchQuery) -> Vec<SearchResult>;
}
