use criterion::{criterion_group, criterion_main, Criterion};
use log::debug;

use dices_rs::dice::parse::parse_with_bonus;
use dices_rs::dice::{Dice, DiceSet};

const FAKE_DICE: &str = "234D20 +23";

fn nom_parser(c: &mut Criterion) {
    let mut ds = DiceSet::from_vec(vec![]);

    c.bench_function("nom_parser", |b| {
        b.iter(|| {
            let r = parse_with_bonus(FAKE_DICE).unwrap();
            ds = r.1;
        })
    });

    let _ = ds;
}

// Old manual parser
//
pub fn parse(s: &str) -> Result<DiceSet, String> {
    let mut ds = Vec::<Dice>::new();
    let mut bonus = 0;

    let uv = s.to_uppercase();

    // split between dice and bonus
    let v: Vec<&str> = uv.split(' ').collect();

    if v.len() == 2 {
        bonus = v[1].parse::<isize>().unwrap_or_default();
    }

    // split dice now
    let mut d: Vec<&str> = v[0].split('D').collect();

    // make it explicit that D6 is 1D6
    d[0] = match d[0] {
        "" => "1",
        _ => d[0],
    };

    let times = d[0].parse::<usize>().unwrap_or_default();
    let size = d[1].parse::<usize>().unwrap();

    for _ in 0..times {
        ds.push(Dice::Regular(size));
    }

    // Insert bonus now if needed
    if bonus != 0 {
        ds.push(Dice::Bonus(bonus));
    }

    Ok(DiceSet::from_vec(ds))
}

fn manual_parser(c: &mut Criterion) {
    let mut ds = DiceSet::from_vec(vec![]);

    c.bench_function("manual_parser", |b| {
        b.iter(|| ds = parse(FAKE_DICE).unwrap())
    });

    let _ = ds;
}

criterion_group!(benches, manual_parser, nom_parser,);

criterion_main!(benches);
