
use crate::internal::internal_roll;
use crate::result::Res;
use crate::roll::*;

fn is_valid(s: usize) -> bool {
    match s {
        4|6|8|10|12|20|100 => true,
        _ => panic!("Error: unknown dice: {}", s),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ConstantDice(usize);

impl Roll for ConstantDice {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        r.append(self.0)
    }
}

/// RegDice is also a usize, only roll() will change
#[derive(Debug, Eq, PartialEq)]
pub struct RegDice(usize);

impl Roll for RegDice {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        r.append(internal_roll(self.0))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct OpenDice {
    pub d: RegDice,
    pub threshold: usize,
}

impl OpenDice {
    pub fn new(s: usize) -> Result<Self,&'static str> {
        if !is_valid(s) {
            return Err("bad size");
        }
        Ok(OpenDice{d: RegDice(s), threshold: s})
    }
}

impl Roll for OpenDice {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        if r.sum >= self.threshold {
            r
        } else {
            r.merge(self.d.roll(&mut Res::new()))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Dice {
    ConstantDice(usize),
    OpenDice(usize),
    RegDice(usize)
}

impl Roll for Dice {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        let s = r.size;

        let res: &Res = match *self {
            Dice::ConstantDice(s) => {
                r.append(self.0);
            },
            Dice::RegDice(s) => {
                r.append(internal_roll(self.0));
            },
            Dice::OpenDice(s) => {
                if r.sum >= self.threshold {
                    r
                } else {
                    r.merge(self.d.roll(&mut Res::new()))
                }
            },
        }
    }

}

#[derive(Debug)]
pub struct DiceSet(Vec<Dice>);

impl Roll for DiceSet {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        let mut r1 = r;

        for dice in &self.0 {
            r1 = dice.roll(r1);
        }
        r1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constant() {
        let mut r = Res::new();

        let f = ConstantDice(0);
        let r = f.roll(&mut r);

        println!("{}", r.sum);
        assert_eq!(1, r.list.len());
    }

    #[test]
    fn test_constant_res() {
        let d= ConstantDice(0);
        let mut r1 = Res::new();
        r1.sum = 2;

        let mut r = Res::new();
        r.sum = 2;

        let r = d.roll(&mut r);

        println!("{:?}", r);
        assert!(r.sum == r1.sum);
        assert_eq!(0, r1.list.len());
        assert_eq!(1, r.list.len());
        assert_eq!(vec![0], r.list);

        let r = d.roll(r);

        assert_eq!(2, r.list.len());
        assert_eq!(vec![0, 0], r.list);
    }

    #[test]
    fn test_is_valid() {
        for i in [4,6,8,10,12,20,100] {
            is_valid(i);
        }
    }

    #[test]
    #[should_panic]
    fn test_not_valid() {
        is_valid(7);
    }

    #[test]
    fn test_reg_new() {
        let a = RegDice(4);

        assert_eq!(4, a.0)
    }

    #[test]
    fn test_reg_roll() {
        let d = RegDice(6);
        let mut r = Res::new();

        assert_eq!(0, r.list.len());
        assert_eq!(0, r.sum);

        let r = d.roll(&mut r);
        println!("{:?}", r);

        assert_eq!(1, r.list.len());
        assert_ne!(0, r.sum);
        assert!(r.sum <= 6)
    }

    #[test]
    fn test_open_new() {
        let d = OpenDice::new(6);

        let d = match d {
            Ok(d) => d,
            _ => panic!("foo")
        };

        let v = d.d;
        assert_eq!(6, v.0);
        assert_eq!(6, d.threshold);
    }

    #[test]
    #[should_panic]
    fn test_open_new_null() {
        OpenDice::new(7);
    }

    #[test]
    fn test_open_roll() {
        let d = RegDice(6);
        let mut r = Res::new();

        assert_eq!(0, r.list.len());
        assert_eq!(0, r.sum);

        let r = d.roll(&mut r);
        println!("{:?}", r);

        assert_eq!(1, r.list.len());
        assert_ne!(0, r.sum);
        assert!(r.sum <= 6)
    }

    #[test]
    fn test_dices_const() {
        let die = Dice::ConstantDice(4);
        let mut r = Res::new();

        let r = die.roll(&mut r);

        println!("{:#?}", r);

        assert_eq!(4, r.sum);
        assert_eq!(4, r.size);
        assert_eq!(0, r.bonus);
    }

    #[test]
    fn test_dices_reg() {
        let die = Dice::RegDice(10);

        println!("{:#?}", die);
    }
}
