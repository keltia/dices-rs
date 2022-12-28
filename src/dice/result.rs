//! Implementation & tests for the structure holding result of rolls
//!
//! All functions returns self to allow for chaining
//!

use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Special {
    None,
    Fumble,
    Natural,
}

/// Holds a result which is all the rolls for a given set of dices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Res {
    /// Store all the rolled dices
    pub list: Vec<usize>,
    /// Sum of all dices
    pub sum: isize,
    /// If there is a malus/bonus to apply
    pub bonus: isize,
    /// Special result?
    pub flag: Special,
}

/// Allow for `.unwrap_or_default()` calls.
impl Default for Res {
    fn default() -> Self {
        Self::new()
    }
}

/// Display trait
impl Display for Res {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "total: {} - incl. bonus: {}", self.sum, self.bonus)
    }
}

/// Our own Res(ult) implementation
impl Res {
    /// Creates an empty dice set.  Assumes all dices are of the same size
    /// although `list` can contains dices of different sizes (cf. `Bonus`).
    ///
    pub fn new() -> Res {
        Res {
            list: Vec::new(),
            sum: 0,
            bonus: 0,
            flag: Special::None,
        }
    }

    /// Add one result to a set
    ///
    pub fn append(&mut self, v: usize) -> &mut Self {
        self.list.push(v);
        self.sum += v as isize;
        self
    }

    /// Merge two sets a & b.  b is empty afterwards.
    ///
    pub fn merge(&mut self, r: &mut Res) -> &mut Self {
        self.list.append(&mut r.list);
        self.sum += r.sum;
        self.bonus += r.bonus;
        self.flag = Special::None;
        self
    }

    /// Do we have a "natural" result?
    ///
    pub fn natural(&self) -> bool {
        self.list.len() == 1 && self.flag == Special::Natural
    }
}

impl Add for Res {
    type Output = Res;

    fn add(self, rhs: Self) -> Self::Output {
        let list = rhs.list.iter().fold(self.list, |mut c, e| {
            c.push(*e);
            c
        });
        Self {
            sum: self.sum + rhs.sum,
            bonus: self.bonus + rhs.bonus,
            flag: Special::None,
            list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Res::new();

        assert_eq!(Special::None, a.flag);
    }

    #[test]
    fn test_append() {
        let mut a = Res {
            list: vec![1],
            sum: 1,
            ..Default::default()
        };

        let b = Res {
            list: vec![1, 2],
            sum: 3,
            ..Default::default()
        };

        let r = a.append(2);
        assert_eq!(&b, r);
    }

    #[test]
    fn test_merge() {
        let mut a = Res {
            list: vec![1],
            sum: 1,
            ..Default::default()
        };

        let mut b = Res {
            list: vec![1, 2],
            sum: 3,
            ..Default::default()
        };

        let r = Res {
            list: vec![1, 1, 2],
            sum: 4,
            ..Default::default()
        };

        let e = a.merge(&mut b);

        assert_eq!(r, *e);
        assert_eq!(0, b.list.len());
    }

    #[test]
    fn test_add() {
        let x = Res {
            list: vec![9, 6],
            sum: 15,
            bonus: 0,
            ..Default::default()
        };
        let y = Res {
            list: vec![],
            sum: 0,
            bonus: -9,
            ..Default::default()
        };

        let s = x + y;
        let t = Res {
            list: vec![9, 6],
            sum: 15,
            bonus: -9,
            ..Default::default()
        };
        assert_eq!(t, s);
    }

    #[test]
    fn test_natural() {
        let a = Res {
            list: vec![1],
            sum: 1,
            ..Default::default()
        };

        assert!(!a.natural());

        let b = Res {
            list: vec![6],
            sum: 6,
            flag: Special::Natural,
            ..Default::default()
        };

        assert!(b.natural());

        let b = Res {
            list: vec![3, 3],
            sum: 6,
            flag: Special::None,
            ..Default::default()
        };

        assert!(!b.natural());
    }
}
