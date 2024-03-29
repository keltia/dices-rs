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
use nom::{
    character::complete::{i8, one_of, space0, u32, u8},
    combinator::{map, opt},
    multi::fold_many0,
    sequence::{pair, preceded},
    IResult,
};

use crate::dice::{Dice, DiceSet};

#[inline]
pub fn parse_dice(input: &str) -> IResult<&str, Dice> {
    let into_dice = |s: u32| Dice::Regular(s as usize);
    let r = preceded(one_of("dD"), u32);
    map(r, into_dice)(input)
}

#[inline]
pub fn parse_open(input: &str) -> IResult<&str, DiceSet> {
    let into_dice = |s: u32| DiceSet::from(Dice::Open(s as usize));
    let r = preceded(one_of("dD"), u32);
    map(r, into_dice)(input)
}

#[inline]
fn parse_ndices(input: &str) -> IResult<&str, DiceSet> {
    let into_set = |(n, d): (Option<std::primitive::u8>, Dice)| {
        let n = n.unwrap_or(1);
        let v: Vec<Dice> = (1..=n).map(|_| d).collect();
        DiceSet::from_vec(v)
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

#[inline]
fn parse_nbonus(input: &str) -> IResult<&str, std::primitive::i8> {
    let sum = |v: Vec<std::primitive::i8>| v.iter().sum1().unwrap_or(0);
    let r = fold_many0(
        preceded(space0, parse_bonus),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    );
    map(r, sum)(input)
}

/// Extracted from parse_with_bonus
///
#[inline]
fn add_bonus((mut ds, b): (DiceSet, std::primitive::i8)) -> DiceSet {
    dbg!(&ds, &b);
    if b != 0 {
        ds.0.push(Dice::Bonus(b.into()))
    };
    ds
}

pub fn parse_open_bonus(input: &str) -> IResult<&str, DiceSet> {
    let r = pair(parse_open, parse_nbonus);
    map(r, add_bonus)(input)
}

pub fn parse_with_bonus(input: &str) -> IResult<&str, DiceSet> {
    let r = pair(parse_ndices, parse_nbonus);
    map(r, add_bonus)(input)
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
    #[case("D6 =2", DiceSet::from_vec(vec ! [Dice::Regular(6)]))]
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
        assert_eq!(out, ds);
    }
}
