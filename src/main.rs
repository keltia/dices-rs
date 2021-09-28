use std::env;
use std::path::PathBuf;

use shelp::{Repl, Color};
use home::home_dir;

mod dice;
mod roll;
mod result;

use crate::result::*;

fn main() {
    let key = "HOME";
    let home = match env::var(key) {
        Ok(val) => val,
        Err(e) => "no HOME".to_string(),
    };

    println!("Hello, world!");

    let r = Res::new();

    println!("{:?}", r);

    let hist: PathBuf =
        [home, ".config".to_string(), "easctl".to_string(), "history".to_string()]
            .iter()
            .collect();

    let mut repl = Repl::newd("EAS> ", ". ", Some(hist));

    loop {
        let cmd = repl.next(Color::Black).unwrap();

        println!("cmd={}", cmd);
    }
}
