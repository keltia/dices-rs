//! nom-based parser for the various pieces of the 'Roll A Dice' grammar.
//!
//! This will be used by the command parser in `main.rs` to validate that the `dice`
//! command is given the right arguments.
//!

use nom::{
    bytes::complete::tag,
    character::complete::{i8, one_of, space0, u32, u8},
    combinator::map,
    combinator::opt,
    sequence::{pair, preceded},
    IResult,
};

use crate::dice::{Dice, DiceSet};

#[inline]
fn parse_dice(input: &str) -> IResult<&str, Dice> {
    let into_dice = |s: u32| Dice::Regular(s as usize);
    let r = preceded(tag("D"), u32);
    map(r, into_dice)(input)
}

#[inline]
fn parse_ndices(input: &str) -> IResult<&str, DiceSet> {
    let into_set = |(n, d): (Option<std::primitive::u8>, Dice)| {
        let n = match n {
            None => 1,
            Some(n) => n,
        };
        let v: Vec<Dice> = (1..=n).map(|_| d).collect();
        DiceSet::from_vec(v.clone())
    };
    let r = pair(opt(u8), parse_dice);
    map(r, into_set)(input)
}

#[inline]
fn parse_bonus(input: &str) -> IResult<&str, std::primitive::i8> {
    let get_sign = |(s, n): (char, i8)| match s {
        '-' => -n,
        '+' => n,
        _ => 0,
    };
    let r = pair(one_of("+-"), i8);
    map(r, get_sign)(input)
}

pub fn parse_with_bonus(input: &str) -> IResult<&str, DiceSet> {
    let add_bonus = |(ds, b): (DiceSet, Option<std::primitive::i8>)| {
        let mut ds = ds.clone();
        if let Some(bonus) = b {
            ds.0.push(Dice::Bonus(bonus.into()))
        };
        ds
    };

    let r = pair(parse_ndices, opt(preceded(space0, parse_bonus)));
    map(r, add_bonus)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::vec;

    #[rstest]
    #[case("D1", DiceSet::from_vec(vec![Dice::Regular(1)]))]
    #[case("D6", DiceSet::from_vec(vec![Dice::Regular(6)]))]
    #[case("3D6", DiceSet::from_vec(vec![Dice::Regular(6), Dice::Regular(6), Dice::Regular(6)]))]
    fn test_parse_dice(#[case] input: &str, #[case] res: DiceSet) {
        let r = parse_ndices(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }

    #[rstest]
    #[case("D1", DiceSet::from_vec(vec![Dice::Regular(1)]))]
    #[case("D6 +2", DiceSet::from_vec(vec![Dice::Regular(6), Dice::Bonus(2)]))]
    #[case("D6 =2", DiceSet::from_vec(vec![Dice::Regular(6)]))]
    #[case("3D6", DiceSet::from_vec(vec![Dice::Regular(6), Dice::Regular(6), Dice::Regular(6)]))]
    #[case("3D6 -2", DiceSet::from_vec(vec![Dice::Regular(6), Dice::Regular(6), Dice::Regular(6), Dice::Bonus(-2)]))]
    fn test_parse_with_bonus(#[case] input: &str, #[case] res: DiceSet) {
        let r = parse_with_bonus(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }
}
