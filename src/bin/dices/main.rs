use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use home::home_dir;
use log::trace;
use rustyline::{config::BellStyle::Visible, CompletionType::List, Config, EditMode, Editor};
use stderrlog::LogLevelNum::{Debug, Info, Trace};

use crate::cli::Opts;
use crate::version::version;

use dices_rs::engine::Engine;
use dices_rs::makepath;

mod cli;
mod version;

const BASE_DIR: &str = ".config";
const ALIASES_FILE: &str = "aliases";
const HISTORY_FILE: &str = "history";

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
        _ => Trace,
    };

    // Prepare logging.
    //
    stderrlog::new()
        .modules(["dices", "dices_rs"])
        .verbosity(lvl)
        .init()
        .unwrap();

    trace!("Load config...");

    // Setup readline
    //
    let cfg = Config::builder()
        .completion_type(List)
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .bell_style(Visible)
        .edit_mode(EditMode::Emacs)
        .build();
    let mut repl = Editor::<()>::with_config(cfg)?;

    // Load history if there is one
    //
    if hist.exists() {
        trace!("Load history from {:?}...", hist);
        repl.load_history(&hist)?;
    }

    // Check whether we supplied an alias file on CLI, if not just load out default one
    //
    trace!("Check for aliases...");
    let alias = match opts.alias_file {
        Some(fname) => Some(PathBuf::from(fname)),
        _ => Some(def_alias),
    };

    // Create a new engine with all builtin commands
    //
    trace!("Create engine...");
    let mut commands = Engine::new().with(alias);

    println!("Available commands:\n{}\n", commands.list());

    match commands.run(&mut repl) {
        Ok(_) => match repl.save_history(&hist) {
            Ok(()) => {
                trace!("Saved history...");
                Ok(())
            }
            Err(e) => Err(anyhow!("Error: can't save history: {}", e.to_string())),
        },
        Err(e) => Err(anyhow!(e.to_string())),
    }
}
