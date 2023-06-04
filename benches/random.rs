//! Benchmark the two RNG toll, the old one and `thread_rng`.
//!

use criterion::{criterion_group, criterion_main, Criterion};
use log::debug;
use rand;
use rand::Rng;

/// Head or Tail?
fn biased_dice(p: f64) -> bool {
    let mut rng = rand::thread_rng();
    let f: f64 = rng.gen();
    f < p
}

/// Return a roll of a dice of size `sides`
///
/// Grows slower as the size of the dice grows
///
pub fn internal_roll_loop(sides: usize) -> usize {
    let mut i = 0;
    loop {
        if biased_dice(1.0 / (sides - i) as f64) {
            return i + 1;
        }
        i += 1;
    }
}

/// Return a roll of a dice of size `sides`
///
/// alternate, `rand` version
///
pub fn internal_roll_rng(sides: usize) -> usize {
    rand::thread_rng().gen_range(1..=sides)
}

fn internal_roll_d20(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("internal_roll/d20", |b| {
        b.iter(|| {
            r = internal_roll_loop(20);
        })
    });
    let _ = r;
}

fn rng_roll_d20(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("rng_roll/d20", |b| {
        b.iter(|| {
            r = internal_roll_rng(20);
        })
    });
    let _ = r;
}

fn internal_roll_d100(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("internal_roll/d100", |b| {
        b.iter(|| {
            r = internal_roll_loop(100);
        })
    });
    let _ = r;
}

fn rng_roll_d100(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("rng_roll/d100", |b| {
        b.iter(|| {
            r = internal_roll_rng(100);
        })
    });
    let _ = r;
}

criterion_group!(
    benches,
    internal_roll_d20,
    rng_roll_d20,
    internal_roll_d100,
    rng_roll_d100
);

criterion_main!(benches);
