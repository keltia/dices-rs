//! nom-based parser for the various pieces of the 'Roll A Dice' grammar.
//!
//! This will be used by the command parser in `main.rs` to validate that the `dice`
//! command is given the right arguments.
//!
//! Not everything is public because there is no need.
//!
//! Public API:
//!
//! - `parse_dice` for a single regular dice
//! - `parse_open` for an open-ended dice
//! - `parse_with_bonus` for regular dices
//! - `parse_open_bonus`  for an open-ended dice

use itertools::Itertools;
use log::trace;
use nom::{
    IResult, Parser,
    character::complete::{i8, one_of, space0, u8, u32},
    combinator::{map_res, opt},
    multi::fold_many0,
    sequence::{pair, preceded},
};
use std::num::ParseIntError;
use std::string::ParseError;

use crate::dice::{Dice, DiceSet};

#[inline]
pub fn parse_dice(input: &str) -> IResult<&str, Dice> {
    let into_dice = |s: u32| -> Result<Dice, ParseIntError> { Ok(Dice::Regular(s as usize)) };
    map_res(preceded(one_of("dD"), u32), into_dice).parse(input)
}

#[inline]
pub fn parse_open(input: &str) -> IResult<&str, DiceSet> {
    let into_dice =
        |s: u32| -> Result<DiceSet, ParseError> { Ok(DiceSet::from(Dice::Open(s as usize))) };
    map_res(preceded(one_of("dD"), u32), into_dice).parse(input)
}

#[inline]
fn parse_ndices(input: &str) -> IResult<&str, DiceSet> {
    let into_set = |(n, d): (Option<std::primitive::u8>, Dice)| -> Result<DiceSet, ParseError> {
        let n = n.unwrap_or(1);
        let v: Vec<Dice> = (1..=n).map(|_| d).collect();
        Ok(DiceSet::from_vec(v))
    };
    map_res(pair(opt(u8), parse_dice), into_set).parse(input)
}

#[inline]
fn parse_bonus(input: &str) -> IResult<&str, std::primitive::i8> {
    let get_sign = |(s, n): (char, i8)| -> Result<i8, ParseError> {
        Ok(match s {
            '-' => -n,
            '+' => n,
            _ => 0,
        })
    };
    map_res(pair(one_of("+-"), i8), get_sign).parse(input)
}

#[inline]
fn parse_nbonus(input: &str) -> IResult<&str, std::primitive::i8> {
    let sum = |v: Vec<std::primitive::i8>| -> Result<i8, ParseIntError> {
        Ok(v.iter().sum1().unwrap_or(0))
    };
    let r = fold_many0(
        preceded(space0, parse_bonus),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    );
    map_res(r, sum).parse(input)
}

/// Extracted from parse_with_bonus
///
#[inline]
fn add_bonus((mut ds, b): (DiceSet, std::primitive::i8)) -> Result<DiceSet, ParseError> {
    trace!("{ds:?}, {b:?}");
    if b != 0 {
        ds.0.push(Dice::Bonus(b.into()))
    };
    Ok(ds)
}

pub fn parse_open_bonus(input: &str) -> IResult<&str, DiceSet> {
    map_res(pair(parse_open, parse_nbonus), add_bonus).parse(input)
}

pub fn parse_with_bonus(input: &str) -> IResult<&str, DiceSet> {
    map_res(pair(parse_ndices, parse_nbonus), add_bonus).parse(input)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("D1", DiceSet::from_vec(vec![Dice::Regular(1)]))]
    #[case("D6", DiceSet::from_vec(vec![Dice::Regular(6)]))]
    #[case("d8", DiceSet::from_vec(vec![Dice::Regular(8)]))]
    #[case("3D6", DiceSet::from_vec(vec![Dice::Regular(6), Dice::Regular(6), Dice::Regular(6)]))]
    fn test_parse_dice(#[case] input: &str, #[case] res: DiceSet) {
        let r = parse_ndices(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }

    #[rstest]
    #[case("D1", DiceSet::from_vec(vec ! [Dice::Regular(1)]))]
    #[case("D6 +2", DiceSet::from_vec(vec ! [Dice::Regular(6), Dice::Bonus(2)]))]
    #[case("D6 +2 +1", DiceSet::from_vec(vec ! [Dice::Regular(6), Dice::Bonus(3)]))]
    #[case("d4 +1", DiceSet::from_vec(vec ! [Dice::Regular(4), Dice::Bonus(1)]))]
    #[case("3D6", DiceSet::from_vec(vec ! [Dice::Regular(6), Dice::Regular(6), Dice::Regular(6)]))]
    #[case("3D6 -2", DiceSet::from_vec(vec ! [Dice::Regular(6), Dice::Regular(6), Dice::Regular(6), Dice::Bonus(- 2)]))]
    fn test_parse_with_bonus(#[case] input: &str, #[case] res: DiceSet) {
        let r = parse_with_bonus(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }

    #[rstest]
    #[case("D6", DiceSet::from(Dice::Open(6)))]
    #[case("d4", DiceSet::from(Dice::Open(4)))]
    fn test_parse_open(#[case] input: &str, #[case] res: DiceSet) {
        let r = parse_open(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }

    #[rstest]
    #[case("", 0)]
    #[case("+1", 1)]
    #[case("-2", - 2)]
    #[case("+1 +2 +3 -2 +7", 11)]
    #[case("+2 +3 +7", 12)]
    #[case(" -1 +2 -2 +7", 6)]
    fn test_parse_nbonus(#[case] input: &str, #[case] sum: i8) {
        let (_input, s) = parse_nbonus(input).unwrap();
        assert_eq!(sum, s);
    }

    #[rstest]
    #[case("d6", DiceSet::from_vec(vec ! [Dice::Open(6)]))]
    #[case("d6 +1", DiceSet::from_vec(vec ! [Dice::Open(6), Dice::Bonus(1)]))]
    #[case("D4 -2", DiceSet::from_vec(vec ! [Dice::Open(4), Dice::Bonus(- 2)]))]
    fn test_parse_open_bonus(#[case] input: &str, #[case] out: DiceSet) {
        let r = parse_open_bonus(input);
        assert!(r.is_ok());
        let (_input, ds) = r.unwrap();
        assert_eq!(out, ds);
    }

    #[rstest]
    #[case(DiceSet(vec ! [Dice::Open(6)]), 0, DiceSet(vec ! [Dice::Open(6)]))]
    #[case(DiceSet(vec ! [Dice::Open(6)]), 1, DiceSet(vec ! [Dice::Open(6), Dice::Bonus(1)]))]
    #[case(DiceSet(vec ! [Dice::Regular(4)]), - 2, DiceSet(vec ! [Dice::Regular(4), Dice::Bonus(- 2)]))]
    fn test_add_bonus(#[case] input: DiceSet, #[case] bonus: i8, #[case] out: DiceSet) {
        let ds = add_bonus((input, bonus));
        assert!(ds.is_ok());
        let ds = ds.unwrap();
        assert_eq!(out, ds);
    }
}
