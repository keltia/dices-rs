use std::path::PathBuf;

use home::home_dir;
use shelp::{Color, Repl};

use dices_rs::dice::result::Res;
use dices_rs::dice::DiceSet;

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
fn main() {
    let home = home_dir().unwrap();

    println!("Hello, world!");

    let hist = makepath!(".config", "dices", "history");

    let mut repl = Repl::newd(PS1, PS2, Some(hist));

    loop {
        let cmd = repl.next(Color::White).unwrap();

        let mut r = Res::new();

        let args: Vec<&str> = cmd.split(' ').collect();
        println!("cmd={}", args[0]);

        if args[0] == "dice" {
            let args = &args[1..];

            let ds = match DiceSet::parse(args[0]) {
                Ok(ds) => ds,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };

            println!("roll = {:?}", ds.roll(&mut r));
        }
    }
}
