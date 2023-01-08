//! Engine module
//!
//! This is where all the CLI parsing is done and stuff is executed.
//!

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::{debug, error, info, trace};
use rustyline::{error::ReadlineError, Editor};

use crate::compiler::{Action, Compiler};

use dices_rs::dice::result::Res;

use self::core::Cmd;

pub mod aliases;
pub mod complete;
pub mod core;

/// This describe all possibilities for commands and aliases
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
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
    pub(crate) cmds: HashMap<String, Command>,
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
        trace!("create compiler with {:?}", self.cmds);
        let cc = Compiler::new(&self.cmds);

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

            let res = match action {
                // Shortcut to exit
                //
                Action::Exit => break,

                // Shortcut to list
                //
                Action::List => {
                    println!("{}", self.list());
                    continue;
                }

                Action::Aliases => {
                    println!("{:?}", self.aliases());
                    continue;
                }

                Action::Macros => {
                    println!("{:?}", self.macros());
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
                Ok(res) => {
                    info!("roll = {:?}", res);
                    debug!("{:?}", res);
                }
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

    /// Call insert() on the inner hash
    ///
    pub fn insert(&mut self, k: String, v: Command) -> &mut Self {
        self.cmds.insert(k, v);
        self
    }

    /// Merge a list of commands into the main engine.
    ///
    pub fn merge(&mut self, aliases: Vec<Command>) -> &mut Self {
        // And merge in aliases
        //
        aliases.iter().for_each(|a| match a {
            Command::Macro { ref name, .. } | Command::Alias { ref name, .. } => {
                self.insert(name.to_owned(), a.to_owned());
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
                format!("{tag}\t{} = {:?}", n, c)
            })
            .join("\n")
    }

    /// Returns all aliases
    ///
    pub fn aliases(&self) -> Vec<String> {
        self.cmds
            .iter()
            .filter_map(|(_name, cmd, ..)| match cmd {
                Command::Alias { name, .. } => Some(name.to_owned()),
                _ => None,
            })
            .collect()
    }

    /// Returns all macros
    ///
    pub fn macros(&self) -> Vec<String> {
        self.cmds
            .iter()
            .filter_map(|(_name, cmd)| match cmd {
                Command::Macro { name, .. } => Some(name.to_owned()),
                _ => None,
            })
            .collect()
    }

    /// Primary aka builtin commands
    ///
    const CMDS: [&'static str; 6] = ["aliases", "dice", "exit", "list", "macros", "open"];

    /// Build a list of `Command` from the builtin commands
    ///
    fn builtin_commands() -> Engine {
        debug!("builtin_commands");
        let all = Self::CMDS.iter().map(|&n| match n {
            // These are caught before
            //
            "aliases" => (n.to_string(), Command::Aliases),
            "exit" => (n.to_string(), Command::Exit),
            "list" => (n.to_string(), Command::List),
            "macros" => (n.to_string(), Command::Macros),
            // General case
            //
            _ => (
                n.to_string(),
                Command::Builtin {
                    name: n.to_string(),
                    cmd: Cmd::from(n),
                },
            ),
        });
        Engine {
            cmds: HashMap::<String, Command>::from_iter(all),
        }
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::engine::Command;

    #[test]
    fn test_builtin_commands() {
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

        let n = Engine::new();
        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }

    #[test]
    fn test_engine_merge() {
        let mut e = Engine::new();

        let doom = vec![Command::Macro {
            name: "doom".to_string(),
            cmd: "2D6".to_string(),
        }];

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
            (
                "doom".to_string(),
                Command::Macro {
                    name: "doom".to_string(),
                    cmd: "2D6".to_string(),
                },
            ),
        ]);

        e.merge(doom);

        assert_eq!(all, e.cmds);
    }

    #[rstest]
    #[case("list", true)]
    #[case("exit", true)]
    #[case("foo", false)]
    fn test_engine_exist(#[case] input: &str, #[case] value: bool) {
        let e = Engine::builtin_commands();
        assert_eq!(value, e.exist(input));
    }

    /// TODO Finish the test
    ///
    #[test]
    fn test_commands_list() {
        let e = Engine::builtin_commands();
        let _str = r##""##;
        dbg!(e.list());
    }
}
