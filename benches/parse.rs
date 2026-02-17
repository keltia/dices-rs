//! Benchmark the two parsers, the current one based on `nom` and the old, manual one.
//!
//! Timing for nom7 and nom8
//! ```text
//! NOM 7
//!
//! manual_parser           time:   [534.36 ns 540.53 ns 547.82 ns]
//!                         change: [−24.416% −22.758% −21.112%] (p = 0.00 < 0.05)
//!                        Performance has improved.
//! Found 10 outliers among 100 measurements (10.00%)
//!   6 (6.00%) high mild
//!   4 (4.00%) high severe
//!
//! nom_parser              time:   [197.28 ns 199.09 ns 201.05 ns]
//!                         change: [−8.3619% −6.6360% −4.9519%] (p = 0.00 < 0.05)
//!                         Performance has improved.
//! Found 7 outliers among 100 measurements (7.00%)
//!   2 (2.00%) low mild
//!   3 (3.00%) high mild
//!   2 (2.00%) high severe
//!
//!
//! NOM 8
//!
//! manual_parser           time:   [530.22 ns 531.63 ns 533.43 ns]
//!                         change: [−20.073% −19.687% −19.308%] (p = 0.00 < 0.05)
//!                         Performance has improved.
//! Found 4 outliers among 100 measurements (4.00%)
//!   1 (1.00%) high mild
//!   3 (3.00%) high severe
//!
//! nom_parser              time:   [185.51 ns 186.16 ns 186.95 ns]
//!                         change: [−7.0377% −6.2565% −5.4446%] (p = 0.00 < 0.05)
//!                         Performance has improved.
//! Found 7 outliers among 100 measurements (7.00%)
//!   3 (3.00%) high mild
//!   4 (4.00%) high severe
//! ```

use criterion::{Criterion, criterion_group, criterion_main};

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
