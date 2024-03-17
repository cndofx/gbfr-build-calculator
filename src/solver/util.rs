use std::collections::HashMap;

use crate::model::{SearchPool, SearchQuery, SearchResult, Trait};

/// filter out sigils and wrightstones that do not have any desired skills
pub fn filter_pool(pool: SearchPool, query: &SearchQuery) -> SearchPool {
    let SearchPool {
        mut sigils,
        mut wrightstones,
    } = pool;

    sigils = sigils
        .into_iter()
        .filter(|sigil| {
            let trait1 = Some(sigil.trait1);
            let trait2 = sigil.trait2;
            trait1.is_some_and(|(t, _)| is_desired_trait(t, query))
                || trait2.is_some_and(|(t, _)| is_desired_trait(t, query))
        })
        .collect();

    wrightstones = wrightstones
        .into_iter()
        .filter(|stone| {
            let trait1 = Some(stone.trait1);
            let trait2 = stone.trait2;
            let trait3 = stone.trait3;
            trait1.is_some_and(|(t, _)| is_desired_trait(t, query))
                || trait2.is_some_and(|(t, _)| is_desired_trait(t, query))
                || trait3.is_some_and(|(t, _)| is_desired_trait(t, query))
        })
        .collect();

    SearchPool {
        sigils,
        wrightstones,
    }
}

/// returns whether the given search result fulfills the search query
pub fn is_valid_result(combo: &SearchResult, query: &SearchQuery) -> bool {
    let mut traits: HashMap<Trait, u8> = HashMap::new();
    for sigil in combo.sigils.iter() {
        let (t1, l1) = sigil.trait1;
        traits.entry(t1).and_modify(|x| *x += l1).or_insert(l1);
        if let Some(trait2) = sigil.trait2 {
            let (t2, l2) = trait2;
            traits.entry(t2).and_modify(|x| *x += l2).or_insert(l2);
        }
    }

    query
        .desired_traits
        .iter()
        .all(|(t, desired_level)| traits.get(t).is_some_and(|n| n >= desired_level))
}

pub fn is_desired_trait(t: Trait, query: &SearchQuery) -> bool {
    query.desired_traits.contains_key(&t)
}
