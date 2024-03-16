use gbfr_build_calculator::model::SearchPool;
use gbfr_build_calculator::solver::{BasicSolver, Solver};
use parser::{parse_sigils, parse_wrightstones};

use crate::parser::parse_query;

mod parser;

fn main() {
    let sigils = std::fs::read_to_string("example_data/sigils.csv").unwrap();
    let (_, sigils) = parse_sigils(&sigils).unwrap();
    dbg!(&sigils);

    let wrightstones = std::fs::read_to_string("example_data/wrightstones.csv").unwrap();
    let (_, wrightstones) = parse_wrightstones(&wrightstones).unwrap();
    dbg!(&wrightstones);

    let query = std::fs::read_to_string("example_data/query.csv").unwrap();
    let (_, query) = parse_query(&query).unwrap();
    dbg!(&query);

    let pool = SearchPool {
        sigils,
        wrightstones,
    };

    let mut results = BasicSolver.search(pool, &query);
    results.sort_by_cached_key(|res| res.total_trait_level());
    for result in &results {
        dbg!(result);
        dbg!(result.total_trait_level());
        println!("\n\n");
    }
    dbg!(&results.len());
}
