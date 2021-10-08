//! Main module to deal with dices, rolls, bonuses, etc.
//!
//! Example:
//! ```
//! let ds = DiceSet::parse("3D6 +1"):
//! let r = Res::new();
//!
//! println!("{:#?}", ds.roll(r));
//! ```

use crate::dice::internal::internal_roll;
use crate::dice::result::Res;

/// Check if we use real dices of fake ones
fn is_valid(s: usize) -> Result<bool, String> {
    match s {
        4|6|8|10|12|20|100 => Ok(true),
        _ => Err(format!("Error: unknown dice: {}", s)),
    }
}

/// Our different types of Dice.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Dice {
    /// Always yield the same result
    Constant(usize),
    /// A dice that will retoll by itself if roll is max
    Open(usize),
    /// Your regular type of dice
    Regular(usize),
    /// Used to register any bonus
    Bonus(isize),
}

/// Implement the dice methods
impl Dice {
    /// Implement `roll()` for each type of dices
    pub fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        let mut res = match *self {
            Dice::Constant(s) => {
                r.append(s)
            },
            Dice::Regular(s) => {
                if !is_valid(s).unwrap() {
                    panic!("Bad size {}", s);
                }

                r.append(internal_roll(s))
            },
            Dice::Open(s) => {
                if !is_valid(s).unwrap() {
                    panic!("Bad size {}", s);
                }

                if r.sum >= s {
                    r
                } else {
                    r.merge(self.roll(&mut Res::new()))
                }
            },
            Dice::Bonus(s) => {
                r.sum += s as usize;
                r
            },
        };
        res.size = self.size();
        res
    }

    /// Return the size of a dice
    fn size(&self) -> usize {
        match *self {
            Dice::Constant(s) |
            Dice::Regular(s) |
            Dice::Open(s) => s,
            Dice::Bonus(_) => 0,
        }
    }
}

/// The more interesting thing, a set of dices
#[derive(Debug, PartialEq)]
pub struct DiceSet(Vec<Dice>);

/// a Dice set
impl DiceSet {
    /// The real stuff, roll every dice in the set and add all rolls
    pub fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        let mut r1 = r;

        for dice in &self.0 {
            match dice {
                Dice::Regular(_) |
                Dice::Open(_) => {
                    r1 = dice.roll(r1);
                },
                Dice::Constant(c) => {
                    r1.sum += *c;
                },
                Dice::Bonus(b) => {
                    r1.bonus += *b;
                }
            }
        }
        r1
    }

    /// Parse a string with the following format:
    ///  `<n>*D<s>[ [+-]<b>+]`
    /// and return a `DiceSet` with `[n * Regular(s), Bonus(b)]`
    pub fn parse(s: &str) -> Result<Self,String> {
        let mut ds = Vec::<Dice>::new();

        let uv = s.to_uppercase();

        // split between dice and bonus
        let v: Vec<&str> = uv
            .split(' ')
            .collect();
        println!("{:?}", v);

        let bonus = isize::from_str_radix(v[1], 10).unwrap_or_default();

        // split dice now
        let mut d: Vec<&str> = v[0]
            .split('D')
            .collect();
        println!("{:?}", d);

        // make it explicit that D6 is 1D6
        d[0] = match d[0] { "" => "1", _ => d[0] };

        let times = usize::from_str_radix(d[0], 10).unwrap_or_default();
        let size= usize::from_str_radix(d[1], 10).unwrap();

        let times = match times { 0 => 1, _ => times, };
        for _ in 0..times {
            ds.push(Dice::Regular(size));
        }

        // Insert bonus now if needed
        if bonus != 0 {
            ds.push(Dice::Bonus(bonus));
        }

        Ok(DiceSet(ds))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Dice::*;
    use rstest::rstest;

    #[test]
    fn test_is_valid() {
        for i in [4,6,8,10,12,20,100] {
            is_valid(i).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_not_valid() {
        let r = match is_valid(7) {
            Ok(r) => r,
            Err(e) => panic!("{}", e)
        };
        assert!(r)
    }

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

        let mut r = Res::new();

        let r = d.roll(&mut r);

        println!("{:?}", r);
        assert!(r.sum == r1.sum);
        assert_eq!(0, r1.list.len());
        assert_eq!(1, r.list.len());
        assert_eq!(vec![6], r.list);

        let r = d.roll(r);

        assert_eq!(2, r.list.len());
        assert_eq!(vec![6, 6], r.list);
    }

    #[test]
    fn test_reg_new() {
        let a = Dice::Regular(4);

        assert_eq!(4, a.size())
    }

    #[test]
    fn test_reg_roll() {
        let d = Dice::Regular(6);
        let mut r = Res::new();

        assert_eq!(0, r.list.len());
        assert_eq!(0, r.sum);

        let r = d.roll(&mut r);

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
        let d = Dice::Regular(6);
        let mut r = Res::new();

        assert_eq!(0, r.list.len());
        assert_eq!(0, r.sum);

        let r = d.roll(&mut r);

        assert_eq!(1, r.list.len());
        assert_ne!(0, r.sum);
        assert!(r.sum <= 6);

        let r = d.roll(r);

        assert_eq!(2, r.list.len());
        println!("{:#?}", r);
        assert!(r.sum >= 2 && r.sum <= 12)

    }

    #[test]
    fn test_dice_const() {
        let die = Dice::Constant(4);
        let mut r = Res::new();

        let r = die.roll(&mut r);

        println!("{:#?}", r);

        assert_eq!(4, r.sum);
        assert_eq!(4, r.size);
        assert_eq!(0, r.bonus);
    }

    #[test]
    fn test_dices() {
        let d1 = Dice::Regular(10);
        let d2 = Dice::Regular(10);
        let d3 = Dice::Bonus(2);

        let v = DiceSet(vec![d1, d2, d3]);

        println!("{:#?}", v);

        let mut r = Res::new();

        let r = v.roll(&mut r);
        println!("{:#?}", r);
        assert!(r.sum >= 4 && r.sum <= 22);
        assert_eq!(2, r.list.len());
        assert_eq!(2, r.bonus);
    }

    #[test]
    fn test_dices_parse() {
        let str = "3D6 +1";

        let ds = match DiceSet::parse(str) {
            Ok(ds) => ds,
            Err(e) => panic!("Unparsable {}", e)
        };

        let rf = DiceSet(vec![Regular(6), Regular(6), Regular(6), Bonus(1)]);

        println!("{:#?}", ds);
        assert_eq!(rf, ds);
    }

    #[test]
    fn test_dices_parse1() {
        let str = "D6 -1";

        let ds = match DiceSet::parse(str) {
            Ok(ds) => ds,
            Err(e) => panic!("Unparsable {}", e)
        };

        let rf = DiceSet(vec![Regular(6), Bonus(-1)]);

        println!("{:#?}", ds);
        assert_eq!(rf, ds);
    }

    #[test]
    fn test_dices_roll() {
        let rf = DiceSet(vec![Regular(6), Regular(6), Regular(6), Bonus(1)]);

        let mut r = Res::new();
        let r = rf.roll(&mut r);

        assert_eq!(1, r.bonus);
        assert_eq!(3, r.list.len())
    }

    #[rstest]
    #[case(Regular(6),6)]
    #[case(Constant(8),8)]
    #[case(Open(12),12)]
    #[case(Bonus(-1),0)]
    fn test_size(#[case] d: Dice, #[case] want: usize) {
        assert_eq!(want, d.size());
    }
}
