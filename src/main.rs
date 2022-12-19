mod cli;
mod version;

use crate::cli::Opts;
use crate::version::version;

use dices_rs::dice::{parse::parse_with_bonus, result::Res};

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use home::home_dir;
use log::{debug, error, info};
use nom::{
    character::complete::{alpha1, space0},
    combinator::map,
    sequence::preceded,
    IResult,
};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use stderrlog::LogLevelNum::{Debug, Info, Trace};

const PS1: &str = "Dices> ";

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
    Doom,
    Exit,
    Invalid(String),
    Move,
    Roll,
}

/// Parse a keyword, return the operation
///
pub fn parse_keyword(input: &str) -> IResult<&str, Cmd> {
    let get_op = |s: &str| match s.to_ascii_lowercase().as_str() {
        "doom" => Cmd::Doom,
        "dice" => Cmd::Roll,
        "mouv" => Cmd::Move,
        "move" => Cmd::Move,
        "roll" => Cmd::Roll,
        "exit" => Cmd::Exit,
        _ => Cmd::Invalid("unknown command".to_string()),
    };
    let r = alpha1;
    map(r, get_op)(input)
}

/// Generic roller
///
fn roll_from(input: &str) -> Result<Res> {
    let mut r = Res::new();

    let ds = match preceded(space0, parse_with_bonus)(input) {
        Ok((_input, ds)) => {
            debug!("{:?}", ds);
            ds
        }
        Err(e) => {
            error!("{:?}", e.to_string());
            return Err(anyhow!("error parsing input"));
        }
    };
    let res = ds.roll(&mut r).clone();
    Ok(res)
}

/// Main entry point
///
fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let home = home_dir().unwrap();
    let hist: PathBuf = makepath!(home, ".config", "dices", "history");

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

    // Prepare logging.
    //
    stderrlog::new().verbosity(lvl).init().unwrap();

    // Setup readline
    //
    let mut repl = Editor::<()>::new()?;

    // Load history f there is one
    //
    if hist.exists() {
        repl.load_history(&hist)?;
    }

    loop {
        // Get next line
        //
        let line = match repl.readline(PS1) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => break,
            Err(e) => {
                error!("{:?}", e);
                break;
            }
        };

        // Save it
        //
        repl.add_history_entry(line.as_str());

        // Get command name
        //
        let line = line.to_ascii_uppercase();
        let (input, cmd) = match parse_keyword(&line) {
            Ok((input, cmd)) => (input, cmd),
            Err(_) => continue,
        };

        debug!("{:?} - {}", cmd, input);

        let res = match cmd {
            Cmd::Doom => roll_from("3D6"),
            Cmd::Move => roll_from("3D6 -9"),
            Cmd::Exit => break,
            Cmd::Roll => roll_from(input),
            _ => {
                error!("Error: unknown command");
                continue;
            }
        };

        match res {
            Ok(res) => {
                info!("roll = {:?}", res);
                debug!("{:?}", res);
            }
            Err(e) => error!("{}", e.to_string()),
        }
    }
    repl.save_history(&hist)?;
    Ok(())
}
