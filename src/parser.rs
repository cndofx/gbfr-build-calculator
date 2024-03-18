use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::{map_res, opt, recognize},
    multi::{many0, separated_list0},
    IResult,
};

use gbfr_build_calculator::model::{SearchQuery, Sigil, Trait, TraitKind, Wrightstone};

pub fn parse_sigils(input: &str) -> IResult<&str, Vec<Sigil>> {
    separated_list0(tag("\n"), sigil)(input)
}

pub fn parse_wrightstones(input: &str) -> IResult<&str, Vec<Wrightstone>> {
    separated_list0(tag("\n"), wrightstone)(input)
}

pub fn parse_query(input: &str) -> IResult<&str, SearchQuery> {
    let (input, sigil_slots) = number(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, desired_traits) = separated_list0(tag("\n"), query_trait)(input)?;

    let desired_traits = desired_traits.into_iter().collect();
    let query = SearchQuery {
        desired_traits,
        sigil_slots,
    };

    Ok((input, query))
}

fn sigil(input: &str) -> IResult<&str, Sigil> {
    let comma = tag(",");

    let (input, trait1) = trait_kind(input)?;
    let (input, _) = comma(input)?;
    let (input, level1) = number(input)?;
    let (input, _) = comma(input)?;
    let (input, trait2) = opt(trait_kind)(input)?;
    let (input, _) = comma(input)?;
    let (input, level2) = opt(number)(input)?;

    let trait1 = Trait {
        kind: trait1,
        level: level1,
    };
    let trait2 = if trait2.is_some() && level2.is_some() {
        Some(Trait {
            kind: trait2.unwrap(),
            level: level2.unwrap(),
        })
    } else {
        None
    };
    let sigil = Sigil { trait1, trait2 };

    Ok((input, sigil))
}

fn wrightstone(input: &str) -> IResult<&str, Wrightstone> {
    let comma = tag(",");

    let (input, trait1) = trait_kind(input)?;
    let (input, _) = comma(input)?;
    let (input, level1) = number(input)?;
    let (input, _) = comma(input)?;
    let (input, trait2) = opt(trait_kind)(input)?;
    let (input, _) = comma(input)?;
    let (input, level2) = opt(number)(input)?;
    let (input, _) = comma(input)?;
    let (input, trait3) = opt(trait_kind)(input)?;
    let (input, _) = comma(input)?;
    let (input, level3) = opt(number)(input)?;

    // let trait1 = (trait1, level1);
    let trait1 = Trait {
        kind: trait1,
        level: level1,
    };
    let trait2 = if trait2.is_some() && level2.is_some() {
        Some(Trait {
            kind: trait2.unwrap(),
            level: level2.unwrap(),
        })
    } else {
        None
    };
    let trait3 = if trait3.is_some() && level3.is_some() {
        Some(Trait {
            kind: trait3.unwrap(),
            level: level3.unwrap(),
        })
    } else {
        None
    };
    let wrightstone = Wrightstone {
        trait1,
        trait2,
        trait3,
    };

    Ok((input, wrightstone))
}

fn query_trait(input: &str) -> IResult<&str, Trait> {
    let comma = tag(",");

    let (input, trait1) = trait_kind(input)?;
    let (input, _) = comma(input)?;
    let (input, level1) = number(input)?;

    let trait1 = Trait {
        kind: trait1,
        level: level1,
    };
    Ok((input, trait1))
}

fn trait_kind(input: &str) -> IResult<&str, TraitKind> {
    map_res(
        recognize(many0(alt((alpha1, space1, tag("\'"))))),
        |s: &str| s.parse::<TraitKind>(),
    )(input)
}

fn number(input: &str) -> IResult<&str, u8> {
    nom::character::complete::u8(input)
}

#[cfg(test)]
mod tests {
    use gbfr_build_calculator::model::TraitSet;
    use TraitKind as TK;

    use super::*;

    // macro_rules! _trait {
    //     ($kind:ident, $level:expr) => {
    //         Trait {
    //             kind: $kind,
    //             level:
    //         }
    //     };
    // }

    #[test]
    fn one_sigil_single_trait() {
        let input = "Aegis,15,,";
        let expected = vec![Sigil {
            trait1: Trait {
                kind: TraitKind::Aegis,
                level: 15,
            },
            trait2: None,
        }];

        let (_, parsed) = parse_sigils(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn one_sigil_double_trait() {
        let input = "Alpha,12,DMG Cap,12";
        let expected = vec![Sigil {
            trait1: Trait {
                kind: TraitKind::Alpha,
                level: 12,
            },
            trait2: Some(Trait {
                kind: TraitKind::DMGCap,
                level: 12,
            }),
        }];

        let (_, parsed) = parse_sigils(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn multi_sigil() {
        let input = "Aegis,15,,\n\
        Alpha,12,DMG Cap,12";

        let expected = vec![
            Sigil {
                trait1: Trait {
                    kind: TraitKind::Aegis,
                    level: 15,
                },
                trait2: None,
            },
            Sigil {
                trait1: Trait {
                    kind: TraitKind::Alpha,
                    level: 12,
                },
                trait2: Some(Trait {
                    kind: TraitKind::DMGCap,
                    level: 12,
                }),
            },
        ];

        let (_, parsed) = parse_sigils(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn multi_wrightstone() {
        let input = "Critical Hit Rate,10,HP,6,Uplift,3\n\
        Stun Power,2,,,,";

        let expected = vec![
            Wrightstone {
                trait1: Trait::new(TK::CriticalHitRate, 10),
                trait2: Some(Trait::new(TK::HP, 6)),
                trait3: Some(Trait::new(TK::Uplift, 3)),
            },
            Wrightstone {
                trait1: Trait::new(TK::StunPower, 2),
                trait2: None,
                trait3: None,
            },
        ];

        let (_, parsed) = parse_wrightstones(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn query_single_trait() {
        let input = "5\n\
        DMG Cap,15";

        let mut desired_traits = TraitSet::new();
        desired_traits.add(Trait::new(TK::DMGCap, 15));

        let expected = SearchQuery {
            desired_traits,
            sigil_slots: 5,
        };

        let (_, parsed) = parse_query(input).unwrap();

        assert_eq!(parsed, expected);
    }
}
