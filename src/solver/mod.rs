use crate::model::{SearchPool, SearchQuery};

mod basic_solver;

pub use basic_solver::BasicSolver;

pub trait Solver {
    fn search(&self, pool: &SearchPool, query: &SearchQuery);
}
