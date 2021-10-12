//! Implementation & tests for the structure holding result of rolls
//!
//! All functions returns self to allow for chaining
//!

/// Holds a result which is all the rolls for a given set of dices.
#[derive(Debug, Eq, PartialEq)]
pub struct Res {
    /// Store all the rolled dices
    pub list: Vec<usize>,
    /// Sum of all dices
    pub sum: usize,
    /// If there is a malus/bonus to apply
    pub bonus: isize,
    /// Assume all same dices
    pub size: usize,
}

/// Allow for `.unwrap_or_default()` calls.
impl Default for Res {
    fn default() -> Self {
        Self::new()
    }
}

/// Our own Res(ult) implementation
impl Res {
    /// Creates an empty dice set.  Assumes all dices are of the same size
    /// although `list` can contains dices of different sizes (cf. `Bonus`).
    pub fn new() -> Res {
        Res {
            list: Vec::new(),
            sum: 0,
            bonus: 0,
            size: 6,
        }
    }

    /// Add one result to a set
    pub fn append(&mut self, v: usize) -> &mut Self {
        self.list.push(v);
        self.sum += v;
        self
    }

    /// Merge two sets a & b.  b is empty afterwards.
    pub fn merge(&mut self, r: &mut Res) -> &mut Self {
        self.list.append(&mut r.list);
        self.sum += r.sum;
        self.bonus += r.bonus;
        self
    }

    /// Do we have a "natural" result?
    pub fn natural(&self) -> bool {
        self.list.len() == 1 && self.sum == self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Res::new();

        assert_eq!(6, a.size);
    }

    #[test]
    fn test_append() {
        let mut a = Res {
            list: vec![1],
            sum: 1,
            bonus: 0,
            size: 6,
        };

        let b = Res {
            list: vec![1, 2],
            sum: 3,
            bonus: 0,
            size: 6,
        };

        let r = a.append(2);
        assert_eq!(&b, r);
    }

    #[test]
    fn test_merge() {
        let mut a = Res {
            list: vec![1],
            sum: 1,
            bonus: 0,
            size: 6,
        };

        let mut b = Res {
            list: vec![1, 2],
            sum: 3,
            bonus: 0,
            size: 6,
        };

        let r = Res {
            list: vec![1, 1, 2],
            sum: 4,
            bonus: 0,
            size: 6,
        };

        let e = a.merge(&mut b);

        assert_eq!(r, *e);
        assert_eq!(0, b.list.len());
    }

    #[test]
    fn test_natural() {
        let a = Res {
            list: vec![1],
            sum: 1,
            bonus: 0,
            size: 6,
        };

        assert!(!a.natural());

        let b = Res {
            list: vec![6],
            sum: 6,
            bonus: 0,
            size: 6,
        };

        assert!(b.natural());
    }
}
