use std::collections::HashMap;

use gbfr_build_calculator::model::{SearchPool, SearchQuery, Sigil, Trait, Wrightstone};
use gbfr_build_calculator::solver::{BasicSolver, Solver};
use parser::parse_sigils;

mod parser;

fn main() {
    let sigils = std::fs::read_to_string("example_data/sigils.csv").unwrap();
    let sigils = parse_sigils(&sigils);
    dbg!(&sigils);
}

// fn main() {
//     let pool = SearchPool {
//         sigils: vec![
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 1),
//             Sigil::new_single(Trait::DMGCap, 10),
//             Sigil::new_single(Trait::DMGCap, 10),
//             Sigil::new_single(Trait::DMGCap, 15),
//             Sigil::new_single(Trait::WarElemental, 15),
//             Sigil::new_single(Trait::PotionHoarder, 11),
//             Sigil::new_single(Trait::PotionHoarder, 6),
//             Sigil::new_single(Trait::WeakPointDMG, 7),
//         ],
//         wrightstones: vec![],
//         // wrightstones: vec![Wrightstone {
//         //     trait1: Trait::WeakPointDMG,
//         //     trait2: Some(Trait::ImprovedHealing),
//         //     trait3: Some(Trait::HP),
//         //     trait1_level: 10,
//         //     trait2_level: 6,
//         //     trait3_level: 5,
//         // }],
//     };

//     // let query = SearchQuery {
//     //     desired_traits: vec![
//     //         (Trait::DMGCap, 22),
//     //         (Trait::WeakPointDMG, 16),
//     //         (Trait::WarElemental, 15),
//     //         (Trait::PotionHoarder, 15),
//     //     ],
//     //     sigil_slots: 10,
//     // };

//     let query = SearchQuery {
//         desired_traits: HashMap::from([
//             (Trait::DMGCap, 17),
//             (Trait::WeakPointDMG, 7),
//             // (Trait::WarElemental, 15),
//             // (Trait::PotionHoarder, 15),
//         ]),
//         sigil_slots: 3,
//     };

//     let results = BasicSolver.search(pool, &query);
//     dbg!(&results);
//     dbg!(&results.len());
// }
