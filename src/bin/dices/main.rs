use std::path::PathBuf;

use clap::Parser;
use colored::*;
use directories::BaseDirs;
use eyre::{Result, eyre};
use log::{info, trace};
use rustyline::{CompletionType::List, Config, EditMode, Editor, config::BellStyle::Visible};
use stderrlog::LogLevelNum::{Debug, Info, Trace};

use crate::cli::Opts;
use crate::version::version;

use dices_rs::{
    Engine,
    compiler::{Action, Compiler},
    complete::DiceCompleter,
};

mod cli;
mod version;

const BASE_DIR: &str = ".config";
const ALIASES_FILE: &str = "aliases";
const HISTORY_FILE: &str = "history";

/// Main entry point
///
fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let base = BaseDirs::new().unwrap();
    let home = base.home_dir();
    let hist = home.join(BASE_DIR).join("dices").join(HISTORY_FILE);
    let def_alias = home.join(BASE_DIR).join("dices").join(ALIASES_FILE);

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
        .init()?;

    trace!("Load config...");

    // Setup readline
    //
    let cfg = Config::builder()
        .completion_type(List)
        .history_ignore_dups(true)?
        .history_ignore_space(true)
        .bell_style(Visible)
        .edit_mode(EditMode::Emacs)
        .build();

    // Create a new engine with all builtin commands
    //
    trace!("Create engine...");
    let mut alias_path = def_alias;
    if let Some(fname) = opts.alias_file {
        alias_path = PathBuf::from(fname);
    }
    let mut commands = Engine::new().with(Some(alias_path.clone())).build();
    println!("{} loaded.\n", alias_path.display());
    let h = DiceCompleter {
        commands: commands.cmds.clone(),
    };
    let mut repl = Editor::with_config(cfg)?;
    repl.set_helper(Some(h));

    // Load history if there is one
    //
    if hist.exists() {
        trace!("Load history from {:?}...", hist);
        repl.load_history(&hist)?;
    }

    // Non-interactive mode
    if !opts.commands.is_empty() {
        let cc = Compiler::new(&commands.cmds);
        for line in opts.commands {
            let action = cc.compile(&line);
            match action {
                Action::Execute(cmd, input) => match cmd.execute(&input) {
                    Ok(res) => println!("{}", res),
                    Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
                },
                Action::Exit => break,
                Action::Error(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                }
                _ => {
                    // Other actions (List, Aliases, Macros) could be implemented too
                    info!("Action {:?} not supported in non-interactive mode", action);
                }
            }
        }
        return Ok(());
    }

    println!("Available commands:\n{}\n", commands.list());

    match commands.run(&mut repl) {
        Ok(_) => match repl.save_history(&hist) {
            Ok(()) => {
                trace!("Saved history...");
                Ok(())
            }
            Err(e) => Err(eyre!("Error: can't save history: {}", e.to_string())),
        },
        Err(e) => Err(eyre!(e.to_string())),
    }
}
