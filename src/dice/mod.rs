//! Main module to deal with dices, rolls, bonuses, etc.
//!
//! We have four types of dices:
//!
//! - `Constant(size)`
//!   always yield the same value
//! - `Regular(size)`
//!   yield a value between 1 and `size`
//! - `Open(size)`
//!   like a regular dice but if value is `size`, reroll one more.
//! - `Bonus(size)`
//!   Simulate a dice to store the bonus along with dices
//!
//! One can use the `Dice` type for individual dices & rolls or the easier `DiceSet` type which
//! has a `parse()` method which simplify the process.
//!
//! Examples:
//! ```
//! use dices_rs::dice::Dice;
//! use dices_rs::dice::result::Res;
//! use dices_rs::dice::Rollable;
//!
//! let d = Dice::Regular(10);
//!
//! println!("{:#?}", d.roll());
//! ```
//!
//! We define a `Res` variable in order to allow method chaining.
//!
//! ```
//! use dices_rs::dice::{DiceSet, Rollable};
//! use dices_rs::dice::result::Res;
//!
//! let ds = match DiceSet::parse("3D6 +1") {
//!     Ok(ds) => ds,
//!     Err(e) => panic!("Error: {}", e)
//! };
//!
//! println!("{:#?}", ds.roll());
//! ```

use crate::dice::result::Special;
use internal::internal_roll;
use parse::parse_with_bonus;
use result::Res;

pub mod internal;
pub mod parse;
pub mod result;

/// Is this thing a Dice or DiceSet?
///
pub trait Rollable {
    fn roll(&self) -> Res;
}

/// Our different types of `Dice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Dice {
    /// Always yield the same result
    Constant(usize),
    /// A dice that will re-roll by itself if roll is max
    Open(usize),
    /// Your regular type of dice
    Regular(usize),
    /// Used to register any bonus, same as a Regular but easier to spot
    Bonus(isize),
}

/// Implement the dice methods
impl Dice {
    /// Return the size of a dice
    ///
    fn size(self) -> usize {
        match self {
            Dice::Constant(s) | Dice::Regular(s) | Dice::Open(s) => s,
            Dice::Bonus(_) => 0,
        }
    }
}

impl Rollable for Dice {
    /// Implement `roll()` for each type of dices
    ///
    fn roll(&self) -> Res {
        let mut r = Res::new();

        let r = match *self {
            Dice::Constant(s) => r.append(s),
            Dice::Regular(s) => {
                let rr = match internal_roll(s) {
                    1 => {
                        r.flag = Special::Fumble;
                        1
                    }
                    s => {
                        r.flag = Special::Natural;
                        s
                    }
                };
                r.append(rr)
            }
            Dice::Open(s) => {
                // While roll is size
                //
                loop {
                    let res = internal_roll(s);
                    r.append(res);
                    if res != s {
                        break;
                    }
                }
                &mut r
            }
            Dice::Bonus(s) => {
                r.sum += s as usize;
                r.bonus = s;
                &mut r
            }
        };
        r.clone()
    }
}

/// The more interesting thing, a set of dices
#[derive(Clone, Debug, PartialEq)]
pub struct DiceSet(Vec<Dice>);

/// a Dice set
impl DiceSet {
    /// Create a DiceSet from a vec of Dice
    /// Used by the nom parser.
    ///
    pub fn from_vec(v: Vec<Dice>) -> Self {
        Self(v)
    }

    /// Add a dice to a `DiceSet`
    ///
    pub fn add(&mut self, d: Dice) -> &mut Self {
        self.0.push(d);
        self
    }

    /// Parse a string with the following format:
    ///  `<n>*D<s>[ [+-]<b>+]`
    /// and return a `DiceSet` with `[n * Regular(s), Bonus(b)]`
    ///
    pub fn parse(s: &str) -> Result<Self, String> {
        match parse_with_bonus(s) {
            Ok((_, ds)) => Ok(ds),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Rollable for DiceSet {
    /// Get all Res and sum them
    ///
    fn roll(&self) -> Res {
        let res = self
            .0
            .iter()
            .map(|d| d.roll())
            .fold(Res::new(), |acc, r| acc + r);
        res.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_constant_new() {
        let f = Dice::Constant(6);

        assert_eq!(6, f.size());
    }

    #[test]
    fn test_constant_roll() {
        let d = Dice::Constant(6);

        let mut r1 = Res::new();
        r1.sum = 6;

        let r = d.roll();

        assert_eq!(r.sum, r1.sum);
        assert_eq!(0, r1.list.len());
        assert_eq!(1, r.list.len());
        assert_eq!(vec![6], r.list);
    }

    #[test]
    fn test_reg_new() {
        let a = Dice::Regular(4);

        assert_eq!(4, a.size())
    }

    #[test]
    fn test_reg_roll() {
        let d = Dice::Regular(6);

        let r = d.roll();

        assert_eq!(1, r.list.len());
        assert_ne!(0, r.sum);
        assert!(r.sum <= 6)
    }

    #[test]
    fn test_open_new() {
        let d = Dice::Open(6);

        assert_eq!(6, d.size());
    }

    #[test]
    fn test_open_roll() {
        let d = Dice::Open(6);

        let r = d.roll();

        match r.list.len() {
            1 => {
                assert_eq!(1, r.list.len());
                assert_ne!(0, r.sum);
                assert!(r.sum <= 6);
            }
            _ => {
                let l = r.list.len();
                assert!(l > 1);
                assert!(r.sum < l * d.size());
            }
        }
    }

    #[test]
    fn test_dice_const() {
        let d = Dice::Constant(4);

        let r = d.roll();

        assert_eq!(4, r.sum);
        assert_eq!(0, r.bonus);
    }

    #[test]
    fn test_dices() {
        let d1 = Dice::Regular(10);
        let d2 = Dice::Regular(10);
        let d3 = Dice::Bonus(2);

        let v = DiceSet::from_vec(vec![d1, d2, d3]);

        let r = v.roll();

        assert!(r.sum >= 4 && r.sum <= 22);
        assert_eq!(2, r.list.len());
        assert_eq!(2, r.bonus);
    }

    #[rstest]
    #[case("D100",vec![Dice::Regular(100)])]
    #[case("D8 -1",vec![Dice::Regular(8), Dice::Bonus(-1)])]
    #[case("3D6 +1",vec![Dice::Regular(6), Dice::Regular(6), Dice::Regular(6), Dice::Bonus(1)])]
    fn test_dices_parse(#[case] d: &str, #[case] v: Vec<Dice>) {
        let ds = match DiceSet::parse(d) {
            Ok(ds) => ds,
            Err(e) => panic!("Unparsable {}", e),
        };

        let rf = DiceSet(v);

        assert_eq!(rf, ds);
    }

    #[test]
    fn test_dices_roll() {
        let rf = DiceSet(vec![
            Dice::Regular(6),
            Dice::Regular(6),
            Dice::Regular(6),
            Dice::Bonus(1),
        ]);

        let r = rf.roll();

        assert_eq!(1, r.bonus);
        assert_eq!(3, r.list.len())
    }

    #[rstest]
    #[case(Dice::Regular(6), 6)]
    #[case(Dice::Constant(8), 8)]
    #[case(Dice::Open(12), 12)]
    #[case(Dice::Bonus(-1),0)]
    fn test_size(#[case] d: Dice, #[case] want: usize) {
        assert_eq!(want, d.size());
    }
}
