//! Engine module
//!
//! This is where all the CLI parsing is done and stuff is executed.
//!

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use log::{debug, error, info, trace};
use nom::{character::complete::alphanumeric1, IResult};
use rustyline::{error::ReadlineError, Editor};

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
pub struct Engine(HashMap<String, Command>);

impl Engine {
    const MAX_RECUR: usize = 5;

    /// Create a new instance
    ///
    pub fn new() -> Self {
        builtin_commands()
    }

    /// Main loop here, refactored from `main()`.
    ///
    pub fn run(&self, repl: &mut Editor<()>) -> Result<()> {
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

            // First analysis
            //
            let (input, cmd) = match self.parse(&line) {
                Ok((input, cmd)) => (input.to_string(), cmd),
                Err(_) => {
                    println!("unknown command");
                    continue;
                }
            };

            trace!("cmd={:?}", cmd);

            // Some actions have to be executed here because they do not involve the "core" dice-related
            // commands and interact with the interactive shell like `exit` and `list`
            //
            let res = match cmd {
                // Shortcut to exit
                //
                Command::Exit => break,

                // Shortcut to list
                //
                Command::List => {
                    println!("{}", self.list());
                    continue;
                }
                // Re-enter the parser until be get to a Builtin
                //
                Command::Macro { cmd, .. } => {
                    trace!("new={}", cmd);

                    // Call recurse with None to use the currently defined max recursion level (5).
                    //
                    let (input, cmd) = match self.recurse(&cmd, None) {
                        Ok((input, cmd)) => (input.to_string(), cmd),
                        Err(e) => {
                            println!("Error: {}", e);
                            continue;
                        }
                    };
                    let res = cmd.execute(&input);
                    res
                }
                // Alias to something that may be a New or Alias
                //
                Command::Alias { cmd, .. } => {
                    trace!("alias = {cmd} {input}");
                    if self.exist(&cmd) {
                        // We have an alias to another command
                        //
                    }
                    let cmd = cmd.to_string() + input.as_str();

                    // Call recurse with None to use the currently defined max recursion level (5).
                    //
                    let (input, cmd) = match self.recurse(&cmd, None) {
                        Ok((input, cmd)) => (input.to_string(), cmd),
                        Err(e) => {
                            println!("Error: {}", e);
                            continue;
                        }
                    };
                    let res = cmd.execute(&input);
                    res
                }
                // These can be executed directly
                //
                Command::Builtin { cmd, .. } => {
                    // Identify and execute each command
                    // Short one may be inserted here directly
                    // otherwise put them in `engine/mod.rs`
                    //
                    trace!("cmd={:?}", cmd);
                    let res = cmd.execute(&input);
                    res
                }
                _ => Err(anyhow!("impossible command")),
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
            // The end, we are at the Builtin level
            //
            Command::Builtin { .. } => {
                trace!("recurse=builtin, end");
                return Ok((input, command));
            }
            // This is an alias
            //
            Command::Alias { cmd, .. } => {
                trace!("recurse=alias({cmd})");
                max -= 1;
                cmd + input.as_str()
            }
            // XXX Need to recurse now but we must not lose any argument so append old input
            //
            Command::Macro { name, cmd } => {
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

    /// Check whether a given command exist
    ///
    pub fn exist(&self, name: &str) -> bool {
        self.0.contains_key(name)
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
        self.0
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
    use rstest::rstest;

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

        assert_eq!(all, e.0);
    }

    #[rstest]
    #[case("list", true)]
    #[case("exit", true)]
    #[case("foo", false)]
    fn test_engine_exist(#[case] input: &str, #[case] value: bool) {
        let e = builtin_commands();
        assert_eq!(value, e.exist(input));
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
