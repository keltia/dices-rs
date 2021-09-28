use crate::roll::*;
use crate::result::Res;

pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100
}

impl Roll for Dice {
    fn roll(t: Dice, n: i32) -> Res {
        todo!()
    }
}
