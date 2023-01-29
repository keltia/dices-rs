//! Engine module
//!
//! This is where all the CLI parsing is done and stuff is executed.
//!

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::{error, info, trace};
use rustyline::{error::ReadlineError, Editor};
use serde::{Deserialize, Serialize};

use crate::compiler::{Action, Compiler};
use crate::dice::result::Res;

use self::core::Cmd;

pub mod aliases;
pub mod complete;
pub mod core;

/// This describe all possibilities for commands and aliases
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
            _ => Err(anyhow!("you can't execute other than Builtin")),
        }
    }
}

const PS1: &str = "Dices> ";

/// Easier to carry around
///
pub struct Engine {
    pub cmds: HashMap<String, Command>,
}

impl Engine {
    /// Create a new instance
    ///
    pub fn new() -> Self {
        Self::builtin_commands()
    }

    /// Main loop here, refactored from `main()`.
    ///
    pub fn run(&mut self, repl: &mut Editor<()>) -> Result<()> {
        let cc = Compiler::new(&self.cmds);

        trace!("Start our input loop");
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

            // Some actions have to be executed here because they do not involve the "core" dice-related
            // commands and interact with the interactive shell like `exit` and `list`
            //
            let action = cc.compile(&line);

            // Now do something with this output of the compiler
            //
            trace!("got ({action:?} as output");
            let res = match action {
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

                    let res = cmd.execute(&input);
                    dbg!(&res);
                    res
                }
                Action::Error(s) => Err(anyhow!("impossible action: {}", s)),
            };
            match res {
                Ok(res) => info!("roll = {:?}", res),
                Err(e) => error!("{}", e.to_string()),
            }
        }
        Ok(())
    }

    /// Check whether a given command exist
    ///
    pub fn exist(&self, name: &str) -> bool {
        self.cmds.contains_key(name)
    }

    /// Merge a list of commands into the main engine.
    ///
    pub fn merge(mut self, aliases: Vec<Command>) -> Self {
        // And merge in aliases
        //
        aliases.iter().for_each(|a| match a {
            Command::Macro { ref name, .. } | Command::Alias { ref name, .. } => {
                self.cmds.insert(name.to_owned(), a.to_owned());
            }
            _ => (),
        });
        self
    }

    /// Lists all available commands
    ///
    pub fn list(&self) -> String {
        self.cmds
            .iter()
            .map(|(n, c)| {
                let tag = match c {
                    Command::Alias { .. } => "alias",
                    Command::Builtin { .. } => "builtin",
                    Command::Macro { .. } => "macro",
                    _ => "special",
                };
                format!("{tag}\t{n} = {c:?}")
            })
            .join("\n")
    }

    /// Returns all aliases
    ///
    pub fn aliases(&self) -> String {
        self.cmds
            .iter()
            .filter_map(|(_name, cmd, ..)| match cmd {
                Command::Alias { name, cmd } => Some((name.to_owned(), cmd)),
                _ => None,
            })
            .map(|(n, c)| format!("alias \t{n} = {c}"))
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
            .map(|(n, c)| format!("macro \t{n} = {c}"))
            .join("\n")
    }

    /// Build a list of `Command` from the builtin commands using a YAML file representing
    /// the list of commands and their type
    ///
    fn builtin_commands() -> Engine {
        trace!("builtin_commands(commands.yaml)");
        let all: HashMap<String, Command> =
            serde_yaml::from_str(include_str!("../bin/dices/commands.yaml")).unwrap();
        Engine { cmds: all }
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
            serde_yaml::from_str(include_str!("../../testdata/builtins.yaml")).unwrap();

        let n = Engine::new();
        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }

    #[test]
    fn test_engine_merge() {
        let n = Engine::new();

        let doom = vec![Command::Macro {
            name: "doom".to_string(),
            cmd: "dice 2D6".to_string(),
        }];

        let all: HashMap<String, Command> =
            serde_yaml::from_str(include_str!("../../testdata/merged.yaml")).unwrap();

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
