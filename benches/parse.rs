use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};

use dices_rs::dice::parse::parse_with_bonus;
use dices_rs::dice::{Dice, DiceSet};

fn nom_parser(c: &mut Criterion) {
    let mut ds = DiceSet::from_vec(vec![]);

    c.bench_function("nom_parser", |b|
        b.iter(|| {
            let r = parse_with_bonus("234D20 +23").unwrap();
            ds = r.1;
        }
        ));

    let _ = ds;
}

fn manual_parser(c: &mut Criterion) {
    let mut ds = DiceSet::from_vec(vec![]);

    c.bench_function("manual_parser", |b|
        b.iter(|| ds = DiceSet::parse("234D20 +23").unwrap()
        ));

    let _ = ds;
}

criterion_group!(
    benches,
    manual_parser,
    nom_parser,
);

criterion_main!(benches);
