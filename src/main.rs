mod aliases;
mod cli;
mod cmds;
mod complete;
mod version;

use crate::aliases::load_aliases;
use crate::cli::Opts;
use crate::cmds::{parse_keyword, roll_from, roll_open, Cmd};
use crate::version::version;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use home::home_dir;
use log::{debug, error, info};
use rustyline::{
    config::BellStyle::Visible, error::ReadlineError, CompletionType::List, Config, Editor,
};
use stderrlog::LogLevelNum::{Debug, Info, Trace};

const BASE_DIR: &str = ".config";
const ALIASES_FILE: &str = "aliases";
const HISTORY_FILE: &str = "history";

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
    let hist: PathBuf = makepath!(&home, BASE_DIR, "dices", HISTORY_FILE);
    let def_alias: PathBuf = makepath!(&home, BASE_DIR, "dices", ALIASES_FILE);

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
    let cfg = Config::builder()
        .completion_type(List)
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .bell_style(Visible)
        .build();
    let mut repl = Editor::<()>::with_config(cfg)?;

    // Check whether we supplied an alias file on CLI
    //
    let alias = match opts.alias_file {
        Some(fname) => PathBuf::from(fname),
        _ => def_alias,
    };

    // Load history if there is one
    //
    if hist.exists() {
        repl.load_history(&hist)?;
    }

    // Load aliases if there is one
    //
    let aliases = match alias.exists() {
        true => load_aliases(alias)?,
        false => vec![],
    };

    debug!("aliases = {:?}", aliases);

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

        // Identify and execute each command
        // Short one may be inserted here directly
        // otherwise put them in `cmds.rs`
        //
        let res = match cmd {
            // Shortcut to exit
            Cmd::Exit => break,
            // Dices of Doom alias
            // TODO: make it a real alias
            Cmd::Doom => roll_from("2D6"),
            // Movement dice
            // TODO: allow bonus
            Cmd::Move => roll_from("3D6 -9"),
            // Open-ended dices
            Cmd::Open => roll_open(input),
            // Regular roll
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
