//! Engine module
//!
//! The engine module is responsible for managing the command-line interface,
//! parsing commands, and executing them. It handles:
//!
//! - Command parsing and execution
//! - Built-in commands management
//! - Macro and alias handling
//! - REPL (Read-Eval-Print Loop) interface
//!

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use eyre::{Result, eyre};
use itertools::Itertools;
use log::{error, trace};
use rustyline::{Editor, error::ReadlineError};
use serde::{Deserialize, Serialize};
use colored::*;
use strsim::levenshtein;

use crate::compiler::{Action, Compiler};
use crate::dice::result::Res;

mod aliases;
mod cmd;
pub mod complete;
mod parse;

pub use cmd::*;
pub use parse::*;

/// Represents all possible command types that can be executed by the engine.
///
/// The Command enum defines the different kinds of commands available:
/// - Macros: User-defined commands stored as strings
/// - Builtins: Core engine commands
/// - Aliases: Alternative names for existing commands
/// - Special commands: Exit, List, etc.
///
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Command {
    /// New command:  define a specific command in a string
    Macro { name: String, cmd: String },
    /// Builtin command
    Builtin { name: String, cmd: Cmd },
    /// Alias of an existing command
    Alias { name: String, cmd: String },
    /// Comment
    Comment,
    /// End of the game
    Exit,
    /// List all commands
    List,
    /// List all aliases
    Aliases,
    /// List all macros
    Macros,
}

impl Command {
    /// Execute defers to `Cmd::execute` for `Builtin`.
    ///
    pub fn execute(&self, input: &str) -> Result<Res> {
        match self {
            Command::Builtin { cmd, .. } => cmd.execute(input),
            _ => Err(eyre!("you can't execute other than Builtin")),
        }
    }
}

/// Core engine that manages commands and their execution.
///
/// The Engine struct maintains a collection of all available commands
/// including built-ins, macros, and aliases. It provides functionality for:
/// - Command execution and management
/// - REPL environment
/// - Command listing and filtering
/// - Alias and macro handling
///
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Engine {
    pub cmds: HashMap<String, Command>,
}

impl Engine {
    /// Create a new instance
    ///
    pub fn new() -> Self {
        Self::builtin_commands()
    }

    /// Run the engine in REPL mode
    ///
    pub fn run(&mut self, repl: &mut Editor<complete::DiceCompleter, rustyline::history::FileHistory>) -> Result<()> {
        let cc = Compiler::new(&self.cmds);

        trace!("Start our input loop");
        loop {
            // Get next line
            //
            let line = match repl.readline(">> ") {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(e) => {
                    error!("{:?}", e);
                    break;
                }
            };

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            trace!("{}", line);

            // Save it
            //
            repl.add_history_entry(line)?;

            // Some actions have to be executed here because they do not involve the "core" dice-related
            // commands and interact with the interactive shell like `exit` and `list`
            //
            let action = cc.compile(line);

            // Now do something with this output of the compiler
            //
            trace!("got ({action:?} as output");
            match action {
                Action::Exit => break,
                Action::List => {
                    println!("{}", self.list());
                    continue;
                }
                Action::Aliases => {
                    println!("{}", self.aliases());
                    continue;
                }
                Action::Macros => {
                    println!("{}", self.macros());
                    continue;
                }
                // Something we can call `execute()` on.
                //
                Action::Execute(cmd, input) => {
                    trace!("exec={:?}", cmd);

                    match cmd.execute(&input) {
                        Ok(res) => {
                            println!("{}", res);
                        }
                        Err(e) => {
                            eprintln!("{}: {}", "Error".red().bold(), e);
                        }
                    }
                }
                Action::Error(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    // Suggest similar commands
                    let words: Vec<_> = line.split_whitespace().collect();
                    if !words.is_empty() {
                        let cmd_name = words[0];
                        let suggestions: Vec<_> = self.cmds.keys()
                            .filter(|name| levenshtein(name, cmd_name) <= 2)
                            .collect();
                        if !suggestions.is_empty() {
                            println!("Did you mean: {}?", suggestions.iter().join(", ").yellow());
                        }
                    }
                }
            };
        }
        Ok(())
    }

    /// Check whether a given command exists
    ///
    pub fn exist(&self, name: &str) -> bool {
        self.cmds.contains_key(name)
    }

    /// Merge a list of commands into the main engine.
    ///
    pub fn merge(&mut self, aliases: Vec<Command>) -> &mut Self {
        // And merge in aliases
        //
        aliases.iter().for_each(|a| match a {
            Command::Macro { name, .. } | Command::Alias { name, .. } => {
                self.cmds.insert(name.to_owned(), a.to_owned());
            }
            _ => (),
        });
        self
    }

    /// Lists all available commands
    ///
    /// Sort on command name.
    ///
    /// TODO: sort inside category (tag)
    ///
    pub fn list(&self) -> String {
        let cmds = self.cmds.keys().sorted();
        cmds.map(|k| {
            let c = self.cmds.get(k).unwrap();
            let tag = match c {
                Command::Alias { .. } => "alias ".blue(),
                Command::Builtin { .. } => "builtin".green(),
                Command::Macro { .. } => "macro ".magenta(),
                _ => "special".yellow(),
            };
            format!("{tag}\t{}", k.bold())
        })
        .join("\n")
    }

    /// Returns all aliases
    ///
    pub fn aliases(&self) -> String {
        self.cmds
            .iter()
            .filter_map(|(_name, cmd)| match cmd {
                Command::Alias { name, cmd } => Some((name.to_owned(), cmd)),
                _ => None,
            })
            .map(|(n, c)| format!("{} \t{n} = {c}", "alias".blue()))
            .join("\n")
    }

    /// Returns all macros
    ///
    pub fn macros(&self) -> String {
        self.cmds
            .iter()
            .filter_map(|(_name, cmd)| match cmd {
                Command::Macro { name, cmd } => Some((name.to_owned(), cmd)),
                _ => None,
            })
            .map(|(n, c)| format!("{} \t{n} = {c}", "macro".magenta()))
            .join("\n")
    }

    /// Build a list of `Command` from the builtin commands using a YAML file representing
    /// the list of commands and their type
    ///
    fn builtin_commands() -> Engine {
        trace!("builtin_commands(commands.yaml)");
        let all: HashMap<String, Command> =
            serde_yml::from_str(include_str!("../bin/dices/commands.yaml")).expect("Invalid commands.yaml");
        Engine { cmds: all }
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Engine({:?})", self.cmds)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::engine::Command;

    use super::*;

    #[test]
    fn test_builtin_commands() {
        // Not using `include_str!` here because it would mean testing if A == A
        //
        let all = HashMap::<String, Command>::from([
            (
                "dice".to_string(),
                Command::Builtin {
                    name: "dice".to_string(),
                    cmd: Cmd::Dice,
                },
            ),
            ("exit".to_string(), Command::Exit),
            ("list".to_string(), Command::List),
            ("aliases".to_string(), Command::Aliases),
            ("macros".to_string(), Command::Macros),
            (
                "open".to_string(),
                Command::Builtin {
                    name: "open".to_string(),
                    cmd: Cmd::Open,
                },
            ),
        ]);

        let n = Engine::builtin_commands();
        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }

    #[test]
    fn test_engine_new() {
        let all: HashMap<String, Command> =
            serde_yml::from_str(include_str!("../../testdata/builtins.yaml")).unwrap();

        let n = Engine::new();
        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }

    #[test]
    fn test_engine_merge() {
        let mut n = Engine::new();

        let doom = vec![Command::Macro {
            name: "doom".to_string(),
            cmd: "dice 2D6".to_string(),
        }];

        let all: HashMap<String, Command> =
            serde_yml::from_str(include_str!("../../testdata/merged.yaml")).unwrap();

        let n = n.merge(doom);

        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }

    #[rstest]
    #[case("list", true)]
    #[case("exit", true)]
    #[case("foo", false)]
    fn test_engine_exist(#[case] input: &str, #[case] value: bool) {
        let e = Engine::builtin_commands();
        assert_eq!(value, e.exist(input));
    }

    #[test]
    fn test_aliases() {
        let e = Engine::builtin_commands();
        let v_str = e.aliases();
        assert!(v_str.is_empty());
    }
}
