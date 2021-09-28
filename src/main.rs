use std::env;
use std::path::PathBuf;

use shelp::{Repl, Color};
use home::home_dir;

mod dice;
mod roll;
mod result;

use crate::result::*;

fn main() {

    let home = home_dir().unwrap();

    println!("Hello, world!");

    let r = Res::new();

    println!("{:?}", r);

    let hist: PathBuf =
        [home,
            PathBuf::from(".config"),
            PathBuf::from("easctl"),
            PathBuf::from("history")
        ]
        .iter()
        .collect();

    let mut repl = Repl::newd("EAS> ", ". ", Some(hist));

    loop {
        let cmd = repl.next(Color::Black).unwrap();

        println!("cmd={}", cmd);
    }
}
