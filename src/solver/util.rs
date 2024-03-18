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
            trait1.is_some_and(|t| is_desired_trait(t, query))
                || trait2.is_some_and(|t| is_desired_trait(t, query))
        })
        .collect();

    wrightstones = wrightstones
        .into_iter()
        .filter(|stone| {
            let trait1 = Some(stone.trait1);
            let trait2 = stone.trait2;
            let trait3 = stone.trait3;
            trait1.is_some_and(|t| is_desired_trait(t, query))
                || trait2.is_some_and(|t| is_desired_trait(t, query))
                || trait3.is_some_and(|t| is_desired_trait(t, query))
        })
        .collect();

    SearchPool {
        sigils,
        wrightstones,
    }
}

/// returns whether the given search result fulfills the search query
pub fn is_valid_result(result: &SearchResult, query: &SearchQuery) -> bool {
    result.traits().is_superset_of(&query.desired_traits)
}

pub fn is_desired_trait(t: Trait, query: &SearchQuery) -> bool {
    query.desired_traits.contains(t.kind)
}
