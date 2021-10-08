use std::path::PathBuf;

use home::home_dir;
use shelp::{Color, Repl};

mod dice;

const PROMPT: &str = "Dices> ";

use dice::result::Res;

/// Main entry point
fn main() {
    let home = home_dir().unwrap();

    println!("Hello, world!");

    let r = Res::new();

    println!("{:?}", r);

    let hist: PathBuf = [
        home,
        PathBuf::from(".config"),
        PathBuf::from("easctl"),
        PathBuf::from("history"),
    ]
    .iter()
    .collect();

    let mut repl = Repl::newd(PROMPT, ". ", Some(hist));

    loop {
        let cmd = repl.next(Color::Black).unwrap();

        println!("cmd={}", cmd);
    }
}
