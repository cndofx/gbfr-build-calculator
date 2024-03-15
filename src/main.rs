use gbfr_build_calculator::model::{SearchPool, SearchQuery, Sigil, Trait, Wrightstone};
use gbfr_build_calculator::solver::{BasicSolver, Solver};

fn main() {
    let pool = SearchPool {
        sigils: vec![
            Sigil::new_single(Trait::DMGCap, 1),
            Sigil::new_single(Trait::DMGCap, 2),
            Sigil::new_single(Trait::DMGCap, 5),
            Sigil::new_single(Trait::DMGCap, 10),
            Sigil::new_single(Trait::DMGCap, 15),
        ],
        wrightstones: vec![Wrightstone {
            trait1: Trait::WeakPointDMG,
            trait2: Some(Trait::ImprovedHealing),
            trait3: Some(Trait::HP),
            trait1_level: 10,
            trait2_level: 6,
            trait3_level: 5,
        }],
    };

    let query = SearchQuery {
        desired_skills: vec![
            (Trait::DMGCap, 22),
            (Trait::WeakPointDMG, 16),
            (Trait::WarElemental, 15),
            (Trait::PotionHoarder, 15),
        ],
        sigil_slots: 12,
    };

    let results = BasicSolver.search(&pool, &query);
}
