
#[derive(Debug,PartialEq)]
pub struct Res {
    pub list: Vec<i32>,
    pub sum: i32,
    pub bonus: i32,
    pub size: i32,
}

impl Res {
    pub fn new() -> Self {
        Res {
            list: Vec::new(),
            sum: 0,
            bonus: 0,
            size: 6,
        }
    }

    pub fn append(&mut self, v: i32) -> &Self {
        self.list.push(v);
        self.sum += v;
        self
    }

    pub fn merge(&mut self, r: &Res) -> &Self {
        for e in &r.list {
            self.list.push(*e);
        }
        self.sum += r.sum;
        self.bonus += r.bonus;
        self
    }

    pub fn natural(&self) -> bool {
        self.list.len() == 1 && &self.sum == &self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_natural() {
        let a = Res {
            list: vec![1],
            sum: 1,
            bonus: 0,
            size: 6,
        };

        assert_eq!(false, a.natural());

        let b = Res {
            list: vec![6],
            sum: 6,
            bonus: 0,
            size: 6,
        };

        assert!(b.natural());
    }
}
