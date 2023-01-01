//! Engine module
//!
//! This is where all the CLI parsing is done and stuff is executed.
//!

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use log::{debug, trace};
use nom::{character::complete::alphanumeric1, IResult};

use self::core::Cmd;

pub mod aliases;
pub mod complete;
pub mod core;

/// This describe all possibilities for commands and aliases
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Command {
    /// New command:  define a specific command in a string
    New { name: String, cmd: String },
    /// Builtin command
    Builtin { name: String, cmd: Cmd },
    /// Alias of an existing command
    Alias { name: String, cmd: Cmd },
    /// Comment
    Comment,
    /// End of the game
    Exit,
    /// List all commands
    List,
}

/// Easier to carry around
///
pub struct Engine(HashMap<String, Command>);

impl Engine {
    const MAX_RECUR: usize = 5;

    /// Create a new instance
    ///
    pub fn new() -> Self {
        builtin_commands()
    }

    /// Parse then validate
    ///
    pub fn parse(&self, input: &str) -> Result<(String, Command)> {
        // Private fn
        //
        fn parse_keyword(input: &str) -> IResult<&str, &str> {
            alphanumeric1(input)
        }

        debug!("all={:?}", self.0);
        // Get command name
        //
        let (input, name) = match parse_keyword(input) {
            Ok((input, name)) => (input.to_owned(), name.to_owned()),
            Err(_) => return Err(anyhow!("invalid command")),
        };

        debug!("{:?} - {}", name, input);

        trace!("name={}", name);

        // Validate that a given input does map to a `Command`
        //
        match self.0.get(&name) {
            Some(cmd) => {
                trace!("found {:?}", cmd);
                Ok((input, cmd.to_owned()))
            }
            None => return Err(anyhow!("unknown command")),
        }
    }

    /// Try to reduce/compile Command::New into a Builtin or Alias
    ///
    /// This is a tail recursive function, might be turned into an iterative one at some point
    /// Not sure it is worth it.
    ///
    pub fn recurse(&self, input: &str, max: Option<usize>) -> Result<(String, Cmd)> {
        trace!("recurse={:?}", input);

        // Set default recursion max
        //
        let mut max = max.unwrap_or(Engine::MAX_RECUR);

        let (input, command) = self.parse(input)?;
        let input = match command {
            // The end, we are at the Builtin or Alias level
            //
            Command::Alias { cmd, .. } | Command::Builtin { cmd, .. } => {
                trace!("recurse=builtin/alias, end");
                return Ok((input, cmd));
            }
            // XXX Need to recurse now but we must not lose any argument so append old input
            //
            Command::New { name, cmd } => {
                trace!("recurse=new({})", name);
                max -= 1;
                cmd + input.as_str()
            }
            _ => bail!("impossible in recurse"),
        };
        // Error out if too deep recursion
        //
        if max == 0 {
            return Err(anyhow!("max recursion level reached for {}", input));
        }
        self.recurse(&input, Some(max))
    }

    /// Call insert() on the inner hash
    ///
    pub fn insert(&mut self, k: String, v: Command) -> &mut Self {
        self.0.insert(k, v);
        self
    }

    /// Merge a list of commands into the main engine.
    ///
    pub fn merge(&mut self, aliases: Vec<Command>) -> &mut Self {
        // And merge in aliases
        //
        aliases.iter().for_each(|a| match a {
            Command::New { ref name, .. } | Command::Alias { ref name, .. } => {
                self.insert(name.to_owned(), a.to_owned());
            }
            _ => (),
        });
        self
    }

    /// Lists all available commands
    ///
    pub fn list(&self) -> String {
        self.0
            .iter()
            .map(|(n, c)| format!("{} = {:?}", n, c))
            .join("\n")
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Primary aka builtin commands
///
const CMDS: [&str; 4] = ["dice", "exit", "list", "open"];

/// Build a list of `Command` from the builtin commands
///
fn builtin_commands() -> Engine {
    debug!("builtin_commands");
    let all: Vec<(String, Command)> = CMDS
        .iter()
        .map(|&n| match n {
            // These are caught before
            //
            "exit" => (n.to_string(), Command::Exit),
            "list" => (n.to_string(), Command::List),
            // General case
            //
            _ => (
                n.to_string(),
                Command::Builtin {
                    name: n.to_string(),
                    cmd: Cmd::from(n),
                },
            ),
        })
        .collect();
    Engine(HashMap::<String, Command>::from_iter(all))
}

#[cfg(test)]
mod tests {
    use super::*;

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
            (
                "open".to_string(),
                Command::Builtin {
                    name: "open".to_string(),
                    cmd: Cmd::Open,
                },
            ),
        ]);

        let b = builtin_commands();
        assert_eq!(all, b.0);
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
            (
                "open".to_string(),
                Command::Builtin {
                    name: "open".to_string(),
                    cmd: Cmd::Open,
                },
            ),
        ]);

        let b = Engine::new();
        assert_eq!(all, b.0);
    }

    #[test]
    fn test_engine_merge() {
        let mut e = Engine::new();

        let doom = vec![Command::New {
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
            (
                "open".to_string(),
                Command::Builtin {
                    name: "open".to_string(),
                    cmd: Cmd::Open,
                },
            ),
            (
                "doom".to_string(),
                Command::New {
                    name: "doom".to_string(),
                    cmd: "2D6".to_string(),
                },
            ),
        ]);

        e.merge(doom);

        assert_eq!(all, e.0);
    }

    /// TODO Finish the test
    ///
    #[test]
    fn test_commands_list() {
        let list = builtin_commands();
        let _str = r##""##;
        dbg!(list.list());
    }
}
