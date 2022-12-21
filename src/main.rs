mod cli;
mod cmds;
mod version;

use crate::cli::Opts;
use crate::cmds::{parse_keyword, roll_from, roll_open, Cmd};
use crate::version::version;

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use home::home_dir;
use log::{debug, error, info};
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
