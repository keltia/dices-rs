

use crate::internal::internal_roll;
use crate::result::Res;
use crate::roll::*;

#[derive(Debug, PartialEq)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

fn is_valid(s: usize) -> Dice {
    match s {
        4 => Dice::D4,
        6 => Dice::D6,
        8 => Dice::D8,
        10 => Dice::D10,
        12 => Dice::D12,
        20 => Dice::D20,
        100 => Dice::D100,
        _ => panic!("Error: unknown dice: {}", s),
    }
}

type ConstantDice = usize;

impl Roll for ConstantDice {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        r.append(*self)
    }
}

#[derive(Debug, PartialEq)]
pub struct RegDice {
    kind: Dice,
    s: usize,
}

impl RegDice {
    pub fn new(s: usize) -> Self {
        Self {
            kind: is_valid(s),
            s: s,
        }
    }
}

impl Roll for RegDice {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        r.append(internal_roll(self.s))
    }
}

#[derive(Debug, PartialEq)]
pub struct OpenDice {
    kind: Dice,
    d: RegDice,
    threshold: usize,
}

impl OpenDice {
    pub fn new(s: usize) -> Self {
        OpenDice {
            kind: is_valid(s),
            d: RegDice::new(s),
            threshold: s,
        }
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

//pub struct Dices<T> {
//    l: Vec<T>
//}

pub type Dices<T> = Vec<T>;

impl Roll for Dices<RegDice> {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        let mut r1 = r;

        for dice in self {
            r1 = dice.roll(r1);
        }
        r1
    }
}

fn new<T>(s: usize) -> Dices<T> {
    <T>::new()
}

/*
impl Dices<OpenDice> {
    pub fn new() -> Self {
        Dices {
            l: Vec::<OpenDice>::new()
        }
    }

    pub fn append(&mut self, iter: impl IntoIterator<Item = OpenDice>) -> &Dices<OpenDice> {
        for d in iter {
            self.l.push(d)
        }
        self
    }
}

impl Roll for Dices<OpenDice> {
    fn roll<'a>(&self, r: &'a mut Res) -> &'a mut Res {
        let mut r1 = r;

        for dice in &self.l {
            r1 = dice.roll(r1);
        }
        r1
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constant() {
        let mut r = Res::new();

        let f: ConstantDice = 0;
        let r = f.roll(&mut r);

        println!("{}", r.sum);
        assert_eq!(1, r.list.len());
    }

    #[test]
    fn test_constant_res() {
        let d: ConstantDice = 0;
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
        let a = RegDice::new(4);

        assert_eq!(4, a.s)
    }

    #[test]
    fn test_reg_roll() {
        let d = RegDice::new(6);
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

        assert_eq!(Dice::D6, d.kind);
        assert_eq!(6, d.threshold);
    }

    #[test]
    #[should_panic]
    fn test_open_new_null() {
        OpenDice::new(7);
    }

    #[test]
    fn test_open_roll() {
        let d = RegDice::new(6);
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
    fn test_dices_reg() {
        let set= Dices::<RegDice>::new();
        let mut r = Res::new();

        let r = set.roll(&mut r);
        println!("{:#?}", r);
    }
}
