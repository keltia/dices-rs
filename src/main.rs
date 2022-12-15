mod cli;
mod version;

use crate::cli::Opts;
use crate::version::version;

use dices_rs::dice::{parse::parse_with_bonus, result::Res};

use std::path::PathBuf;

use anyhow::anyhow;
use clap::Parser;
use home::home_dir;
use log::{debug, error, info};
use nom::{
    character::complete::{alpha1, space0},
    combinator::map,
    sequence::preceded,
    IResult,
};
use shelp::{Color, Repl};
use stderrlog::LogLevelNum::{Debug, Info, Trace};

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
    let opts: Opts = Opts::parse();

    let home = home_dir().unwrap();
    let hist = makepath!(home, ".config", "dices", "history");

    // Add banner
    //
    println!("{}\n", version());

    // Exit if needed
    //
    if opts.version {
        std::process::exit(0);
    }

    // Check verbosity
    //
    let lvl = match opts.verbose {
        0 => Info,
        1 => Debug,
        2 => Trace,
        _ => Trace,
    };

    // If we use colours, use light/dark modes
    //
    let colour = if opts.dark {
        Color::White
    } else {
        Color::Black
    };

    // Prepare logging.
    //
    stderrlog::new().verbosity(lvl).init().unwrap();

    let repl = Repl::newd(PS1, PS2, Some(hist));

    for line in repl.iter(colour) {
        let mut r = Res::new();
        let line = line.to_ascii_uppercase();

        let (input, cmd) = match parse_keyword(&line) {
            Ok((input, cmd)) => (input, cmd),
            Err(e) => {
                error!("Error: {}", anyhow!("{}", e.to_string()));
                continue;
            }
        };

        debug!("{:?} - {}", cmd, input);

        match cmd {
            Cmd::Roll => {
                debug!("now roll it!");
                let ds = match preceded(space0, parse_with_bonus)(input) {
                    Ok((_input, ds)) => {
                        debug!("{:?}", ds);
                        ds
                    }
                    Err(e) => {
                        debug!("Error:{}", anyhow!("{}", e.to_string()));
                        continue;
                    }
                };
                let res = ds.roll(&mut r);
                info!("roll = {}", res);
                debug!("{:?}", res);
            }
            _ => error!("Error: unknown command"),
        }
    }
}
