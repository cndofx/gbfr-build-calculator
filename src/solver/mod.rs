// inspired by https://github.com/TanukiSharp/MHArmory/blob/master/MHArmory.Search/Documentation/README.md

use crate::model::{SearchPool, SearchQuery, SearchResult};

mod basic_solver;

pub use basic_solver::BasicSolver;

pub trait Solver {
    fn search(&self, pool: &SearchPool, query: &SearchQuery) -> Vec<SearchResult>;
}
