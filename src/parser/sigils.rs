use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::{map_res, opt, recognize},
    multi::{many0, separated_list0},
    IResult,
};

use gbfr_build_calculator::model::{Sigil, Trait};

pub fn parse_sigils(input: &str) -> IResult<&str, Vec<Sigil>> {
    separated_list0(tag("\n"), sigil)(input)
}

fn sigil(input: &str) -> IResult<&str, Sigil> {
    let comma = tag(",");

    let (input, trait1) = trait_name(input)?;
    let (input, _) = comma(input)?;
    let (input, level1) = trait_level(input)?;
    let (input, _) = comma(input)?;
    let (input, trait2) = opt(trait_name)(input)?;
    let (input, _) = comma(input)?;
    let (input, level2) = opt(trait_level)(input)?;

    let trait1 = (trait1, level1);
    let trait2 = if trait2.is_some() && level2.is_some() {
        Some((trait2.unwrap(), level2.unwrap()))
    } else {
        None
    };
    let sigil = Sigil { trait1, trait2 };

    Ok((input, sigil))
}

fn trait_name(input: &str) -> IResult<&str, Trait> {
    map_res(
        recognize(many0(alt((alpha1, space1, tag("\'"))))),
        |s: &str| s.parse::<Trait>(),
    )(input)
}

fn trait_level(input: &str) -> IResult<&str, u8> {
    let a = nom::character::complete::u8(input);
    a
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_single_trait() {
        let input = "Aegis,15,,";
        let expected = vec![Sigil {
            trait1: (Trait::Aegis, 15),
            trait2: None,
        }];

        let (_, parsed) = parse_sigils(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn one_double_trait() {
        let input = "Alpha,12,DMG Cap,12";
        let expected = vec![Sigil {
            trait1: (Trait::Alpha, 12),
            trait2: Some((Trait::DMGCap, 12)),
        }];

        let (_, parsed) = parse_sigils(input).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    fn multi() {
        let input = "Aegis,15,,\n\
        Alpha,12,DMG Cap,12";

        let expected = vec![
            Sigil {
                trait1: (Trait::Aegis, 15),
                trait2: None,
            },
            Sigil {
                trait1: (Trait::Alpha, 12),
                trait2: Some((Trait::DMGCap, 12)),
            },
        ];

        let (_, parsed) = parse_sigils(input).unwrap();

        assert_eq!(parsed, expected);
    }
}
