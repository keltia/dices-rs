use crate::dice::Dice;
use crate::result::Res;

pub trait Roll {
    fn roll(t: Dice, n: i32) -> Res;
}

