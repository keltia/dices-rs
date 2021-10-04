use rand::prelude::*;

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

struct OpenDice;

fn biased_dice(p: f64) -> bool {
    let mut rng = rand::thread_rng();
    let f: f64 = rng.gen();
    f < p
}

fn internal_roll(sides: i32) -> i32 {
    let mut i = 0;
    loop {
        if biased_dice(1.0 / (sides - i) as f64) {
            return i + 1
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_internal_roll() {
        for i in 0..10 {
            let r = internal_roll(6);

            assert!(r >= 0 && r <= 6)
        }
    }
}


