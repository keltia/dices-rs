//! Main rolling generator
//!
//! Example:
//! ```
//! use dices_rs::internal::internal_roll;
//!
//! let r = internal_roll(6);
//!
//! println!("Roll = {}", r);
//! ```
//!

/// Include the rand family
use rand::prelude::*;

/// Head or Tail?
fn biased_dice(p: f64) -> bool {
    let mut rng = rand::thread_rng();
    let f: f64 = rng.gen();
    f < p
}

/// Return a roll of a dice of size `sides`
pub fn internal_roll(sides: usize) -> usize {
    let mut i = 0;
    loop {
        if biased_dice(1.0 / (sides - i) as f64) {
            return i + 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_roll() {
        for _i in 0..10 {
            let r = internal_roll(6);

            assert!(r <= 6)
        }
    }
}
