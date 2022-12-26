use dices_rs::dice::{
    parse::{parse_open, parse_with_bonus},
    result::Res,
    Rollable,
};
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use log::{debug, error, trace};
use nom::{character::complete::space0, sequence::preceded};

/// This describe all possibilities for commands anad aliases
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
}

/// List of builtin commands
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Cmd {
    /// Roll of dices
    Dice,
    /// End of program
    Exit,
    /// Invalid command
    Invalid,
    /// Define a new command
    New,
    /// Roll an open dice
    Open,
}

impl From<&str> for Cmd {
    /// Return the command associated with the keyword (excluding aliases)
    ///
    fn from(value: &str) -> Self {
        match value {
            "dice" => Cmd::Dice,
            "exit" => Cmd::Exit,
            "new" => Cmd::New,
            "open" => Cmd::Open,
            _ => Cmd::Invalid,
        }
    }
}

/// Primary aka builtin commands
///
const CMDS: [&str; 4] = ["dice", "exit", "open", "invalid"];

/// Build a list of `Command` from the builtin commands
///
pub fn builtin_commands() -> HashMap<String, Command> {
    debug!("builtin_commands");
    let all: Vec<(String, Command)> = CMDS
        .iter()
        .map(|&n| {
            if n == "exit" {
                ("exit".to_string(), Command::Exit)
            } else {
                (
                    n.to_string(),
                    Command::Builtin {
                        name: n.to_string(),
                        cmd: Cmd::from(n),
                    },
                )
            }
        })
        .collect();
    HashMap::<String, Command>::from_iter(all)
}

pub fn validate_command(commands: &HashMap<String, Command>, name: &str) -> Result<Command> {
    match commands.get(name) {
        Some(cmd) => Ok(cmd.to_owned()),
        None => Err(anyhow!("unknown command")),
    }
}

/// Generic roller
///
pub fn roll_from(input: &str) -> Result<Res> {
    trace!("roll_from");
    let ds = match preceded(space0, parse_with_bonus)(input) {
        Ok((_input, ds)) => {
            debug!("{:?}", ds);
            ds
        }
        Err(e) => {
            error!("{:?}", e.to_string());
            return Err(anyhow!("error parsing input"));
        }
    };
    Ok(ds.roll())
}

/// Generic open dice roller
///
pub fn roll_open(input: &str) -> Result<Res> {
    trace!("roll_open");
    let d = match preceded(space0, parse_open)(input) {
        Ok((_input, d)) => {
            debug!("{:?}", d);
            d
        }
        Err(e) => {
            error!("{:?}", e.to_string());
            return Err(anyhow!("error parsing input"));
        }
    };
    Ok(d.roll())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmds::Cmd;

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
            (
                "exit".to_string(),
                Command::Builtin {
                    name: "exit".to_string(),
                    cmd: Cmd::Exit,
                },
            ),
            (
                "open".to_string(),
                Command::Builtin {
                    name: "open".to_string(),
                    cmd: Cmd::Open,
                },
            ),
            (
                "invalid".to_string(),
                Command::Builtin {
                    name: "invalid".to_string(),
                    cmd: Cmd::Invalid,
                },
            ),
        ]);

        let b = builtin_commands();
        assert_eq!(all, b);
    }
}
