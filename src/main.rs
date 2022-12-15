use dices_rs::dice::{parse::parse_with_bonus, result::Res};

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use home::home_dir;
use shelp::{Color, Repl};

const PS1: &str = "Dices> ";
const PS2: &str = "..> ";

/// Simple macro to generate PathBuf from a series of entries
///
#[macro_export]
macro_rules! makepath {
    ($($item:expr),+) => {
        [
        $(PathBuf::from($item),)+
        ]
        .iter()
        .collect()
    };
}

use nom::{
    character::complete::{alpha1, space0},
    combinator::map,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub enum Cmd {
    Invalid(String),
    Roll,
}

/// Parse a keyword, return the operation
///
pub fn parse_keyword(input: &str) -> IResult<&str, Cmd> {
    let get_op = |s: &str| match s.to_ascii_lowercase().as_str() {
        "dice" => Cmd::Roll,
        _ => Cmd::Invalid("unknown command".to_string()),
    };
    let r = alpha1;
    map(r, get_op)(input)
}

/// Main entry point
///
fn main() {
    let home = home_dir().unwrap();
    let hist = makepath!(home, ".config", "dices", "history");

    let repl = Repl::newd(PS1, PS2, Some(hist));

    for line in repl.iter(Color::White) {
        let mut r = Res::new();
        let line = line.to_ascii_uppercase();

        let (input, cmd) = match parse_keyword(&line) {
            Ok((input, cmd)) => (input, cmd),
            Err(e) => {
                println!("Error(parse)");
                continue;
            }
        };

        println!("{:?} - {}", cmd, input);

        match cmd {
            Cmd::Roll => {
                println!("now roll it!");
                let ds = match preceded(space0, parse_with_bonus)(input) {
                    Ok((_input, ds)) => {
                        println!("{:?}", ds);
                        ds
                    }
                    Err(e) => {
                        println!("Error(roll)");
                        continue;
                    }
                };
                println!("roll = {:?}", ds.roll(&mut r));
            }
            _ => println!("Error: unknown command"),
        }
    }
}
