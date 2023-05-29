//! Benchmark the two RNG toll, the old one and `thread_rng`.
//!

use criterion::{criterion_group, criterion_main, Criterion};
use log::debug;

use dices_rs::dice::internal::{internal_roll, rng_roll};

fn internal_roll_d20(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("internal_roll/d20", |b| {
        b.iter(|| {
            r = internal_roll(20);
        })
    });
    let _ = r;
}

fn rng_roll_d20(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("rng_roll/d20", |b| {
        b.iter(|| {
            r = rng_roll(20);
        })
    });
    let _ = r;
}

fn internal_roll_d100(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("internal_roll/d100", |b| {
        b.iter(|| {
            r = internal_roll(100);
        })
    });
    let _ = r;
}

fn rng_roll_d100(c: &mut Criterion) {
    let mut r: usize = 0;

    c.bench_function("rng_roll/d100", |b| {
        b.iter(|| {
            r = rng_roll(100);
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
