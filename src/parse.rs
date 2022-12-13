use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, i32, i8, one_of, space0, u32, u8},
    combinator::map,
    combinator::{opt, recognize},
    multi::many0,
    sequence::delimited,
    sequence::{pair, preceded, tuple},
    IResult,
};

use dices_rs::dice::{Dice, DiceSet};

fn parse_dice(input: &str) -> IResult<&str, Dice> {
    let into_dice = |s: u32| Dice::Constant(s as usize);
    let r = preceded(tag("D"), u32);
    map(r, into_dice)(input)
}

fn parse_ndices(input: &str) -> IResult<&str, DiceSet> {
    let into_set = |(n, d): (Option<std::primitive::u8>, Dice)| {
        let n = match n {
            None => 1,
            Some(n) => n,
        };
        let v: Vec<Dice> = (1..=n).map(|f| d).collect();
        DiceSet::from_vec(v.clone())
    };
    let r = pair(opt(u8), parse_dice);
    map(r, into_set)(input)
}

fn parse_bonus(input: &str) -> IResult<&str, std::primitive::i8> {
    preceded(one_of("+-"), i8)(input)
}

fn parse_with_bonus(input: &str) -> IResult<&str, (DiceSet, i8)> {
    let bonus = |(d, b): (DiceSet, Option<std::primitive::i8>)| {
        let b: i8 = match b {
            None => 0,
            Some(n) => n,
        };
        (d, b)
    };

    let r = pair(parse_ndices, opt(preceded(space0, parse_bonus)));
    map(r, bonus)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::vec;

    #[rstest]
    #[case("D1", DiceSet::from_vec(vec![Dice::Constant(1)]))]
    #[case("D6", DiceSet::from_vec(vec![Dice::Constant(6)]))]
    #[case("3D6", DiceSet::from_vec(vec![Dice::Constant(6), Dice::Constant(6), Dice::Constant(6)]))]
    fn test_parse_dice(#[case] input: &str, #[case] res: DiceSet) {
        let r = parse_ndices(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }

    #[rstest]
    #[case("D1", (DiceSet::from_vec(vec![Dice::Constant(1)]), 0))]
    #[case("D6 +2", (DiceSet::from_vec(vec![Dice::Constant(6)]), 2))]
    #[case("3D6", (DiceSet::from_vec(vec![Dice::Constant(6), Dice::Constant(6), Dice::Constant(6)]), 0))]
    #[case("3D6 -2", (DiceSet::from_vec(vec![Dice::Constant(6), Dice::Constant(6), Dice::Constant(6)]), -2))]
    fn test_parse_with_bonus(#[case] input: &str, #[case] res: (DiceSet, i8)) {
        let r = parse_with_bonus(input);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(res, r.1);
    }
}
