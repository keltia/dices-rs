mod cmds;
use cmds::{parse_line, Cmd};

use dices_rs::dice::{parse::parse_with_bonus, result::Res};

use std::path::PathBuf;

use anyhow::Result;
use home::home_dir;
use nom::{combinator::all_consuming, Finish};
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
/// Main entry point
///
fn main() -> Result<()> {
    let home = home_dir().unwrap();

    println!("Hello, world!");

    let hist = makepath!(home, ".config", "dices", "history");

    let mut repl = Repl::newd(PS1, PS2, Some(hist));

    loop {
        let line = repl.next(Color::White).unwrap();

        let mut r = Res::new();

        if let Ok((_null, cmd)) = all_consuming(parse_line)(&line).finish() {
            match cmd {
                Cmd::WithArg { op, args } => {
                    let (_input, ds) = parse_with_bonus(args)?;
                    println!("roll = {:?}", ds.roll(&mut r));
                }
                _ => println!("Error: unknown command"),
            }
        }
    }
    Ok(())
}
