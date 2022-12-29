use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use home::home_dir;
use log::{debug, error, info, trace};
use rustyline::{
    config::BellStyle::Visible, error::ReadlineError, CompletionType::List, Config, Editor,
};
use stderrlog::LogLevelNum::{Debug, Info, Trace};

use crate::aliases::load_aliases;
use crate::cli::Opts;
use crate::cmds::Command;
use crate::engine::Engine;
use crate::version::version;

mod aliases;
mod cli;
mod cmds;
mod complete;
mod core;
mod engine;
mod version;

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
    stderrlog::new()
        .modules(["core", "dices_rs", "dice"])
        .verbosity(lvl)
        .init()
        .unwrap();

    debug!("Load config...");

    // Setup readline
    //
    let cfg = Config::builder()
        .completion_type(List)
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .bell_style(Visible)
        .build();
    let mut repl = Editor::<()>::with_config(cfg)?;

    // Load history if there is one
    //
    if hist.exists() {
        debug!("Load history from {:?}...", hist);
        repl.load_history(&hist)?;
    }

    // Check whether we supplied an alias file on CLI, if not just load out default one
    //
    let alias = match opts.alias_file {
        Some(fname) => Some(PathBuf::from(fname)),
        _ => Some(def_alias),
    };

    // Load aliases if there is one.  If no file or nothing new, return the builtin aliases
    //
    let aliases = load_aliases(alias)?;
    debug!("aliases = {:?}", aliases);

    // Create a new engine with all builtin commands
    //
    let mut commands = Engine::new();

    // And merge in aliases
    //
    let commands = commands.merge(aliases);
    debug!("commands = {:?}", commands);

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

        trace!("{}", line);

        // Save it
        //
        repl.add_history_entry(line.as_str());

        let (input, cmd) = commands.parse(&line)?;

        debug!("{:?}", cmd);

        // Shortcut to exit
        //
        if cmd == Command::Exit {
            break;
        }

        // Shortcut to list
        //
        if cmd == Command::List {
            println!("{}", commands.list());
            continue;
        }

        // Identify and execute each command
        // Short one may be inserted here directly
        // otherwise put them in `cmds.rs`
        //
        let res = cmd.execute(&input);

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
