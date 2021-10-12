use std::path::PathBuf;

use home::home_dir;
use shelp::{Color, Repl};

mod dice;

const PS1: &str = "Dices> ";
const PS2: &str = "..> ";

use dices_rs::dice::*;
use dices_rs::result::Res;

/// Main entry point
fn main() {
    let home = home_dir().unwrap();

    println!("Hello, world!");

    let hist: PathBuf = [
        home,
        PathBuf::from(".config"),
        PathBuf::from("easctl"),
        PathBuf::from("history"),
    ]
    .iter()
    .collect();

    let mut repl = Repl::newd(PS1, PS2, Some(hist));

    loop {
        let cmd = repl.next(Color::Black).unwrap();

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
                },
            };

            println!("roll = {:?}", ds.roll(&mut r));
        }
    }
}
