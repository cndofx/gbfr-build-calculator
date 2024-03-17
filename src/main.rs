use gbfr_build_calculator::model::SearchPool;
use gbfr_build_calculator::solver::{GeneticSolver, NaiveSolver, Solver};
use parser::{parse_sigils, parse_wrightstones};

use crate::parser::parse_query;

mod parser;

fn main() {
    let sigils = std::fs::read_to_string("example_data/sigils.csv").unwrap();
    let (_, sigils) = parse_sigils(&sigils).unwrap();
    // dbg!(&sigils);

    let wrightstones = std::fs::read_to_string("example_data/wrightstones.csv").unwrap();
    let (_, wrightstones) = parse_wrightstones(&wrightstones).unwrap();
    // dbg!(&wrightstones);

    let query = std::fs::read_to_string("example_data/query.csv").unwrap();
    let (_, query) = parse_query(&query).unwrap();
    // dbg!(&query);

    let pool = SearchPool {
        sigils,
        wrightstones,
    };

    // let mut results = NaiveSolver.search(pool, &query);
    // results.sort_by_cached_key(|res| res.total_trait_level());

    // for (i, result) in results.iter().enumerate() {
    //     println!("Result {}", i + 1);
    //     println!("\nSigils:");
    //     for sigil in &result.sigils {
    //         println!("{}", sigil);
    //     }
    //     println!("\nWrightstone:");
    //     if let Some(stone) = &result.wrightstone {
    //         println!("{}", stone);
    //     } else {
    //         println!("None")
    //     }
    //     println!("\nTotal Trait Level: {}", result.total_trait_level());
    //     println!("\n\n\n");
    // }

    // println!("{} results found.", results.len());

    let solver = GeneticSolver {
        population_size: 1,
        generations: 1,
        crossover_rate: 0.6,
        mutation_rate: 0.05,
    };
    let results = solver.search(pool, &query);
    todo!()
}
