use dices_rs::dice::{
    parse::{parse_open, parse_with_bonus},
    result::Res,
    Rollable,
};
use std::collections::HashMap;

use crate::aliases::Alias;
use anyhow::{anyhow, Result};
use log::{debug, error, trace};
use nom::{
    character::complete::{alpha1, space0},
    combinator::map,
    sequence::preceded,
    IResult,
};

/// List of existing commands without aliases
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Cmd {
    Dice,
    Doom,
    Exit,
    Invalid,
    Open,
}

impl From<&str> for Cmd {
    /// Return the command associated with the keyword (excluding aliases)
    ///
    fn from(value: &str) -> Self {
        match value {
            "dice" => Cmd::Dice,
            "exit" => Cmd::Exit,
            "open" => Cmd::Open,
            _ => Cmd::Invalid,
        }
    }
}

/// Primary aka builtin commands
///
const CMDS: [&str; 4] = ["dice", "exit", "open", "invalid"];

/// Build a list of "aliases" from the builtin commands
///
pub fn builtin_commands() -> HashMap<String, Alias> {
    debug!("builtin_commands");
    let all: Vec<(Cmd, Alias)> = CMDS
        .iter()
        .map(|&n| {
            (
                n.to_string(),
                Alias::Command {
                    name: n.to_string(),
                    cmd: Cmd::from(n),
                },
            )
        })
        .collect();
    HashMap::<String, Alias>::from_iter(all)
}

/// Parse a keyword, return the operation
///
pub fn parse_keyword(input: &str) -> IResult<&str, Cmd> {
    trace!("parse_keyword");
    let get_op = |s: &str| match s.to_ascii_lowercase().as_str() {
        "doom" => Cmd::Doom,
        "dice" => Cmd::Dice,
        "open" => Cmd::Open,
        "roll" => Cmd::Dice,
        "exit" => Cmd::Exit,
        _ => Cmd::Invalid,
    };
    let r = alpha1;
    map(r, get_op)(input)
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
        let all = HashMap::<String, Alias>::from([
            (
                "dice".to_string(),
                Alias::Command {
                    name: "dice".to_string(),
                    cmd: Cmd::Dice,
                },
            ),
            (
                "exit".to_string(),
                Alias::Command {
                    name: "exit".to_string(),
                    cmd: Cmd::Exit,
                },
            ),
            (
                "open".to_string(),
                Alias::Command {
                    name: "open".to_string(),
                    cmd: Cmd::Open,
                },
            ),
            (
                "invalid".to_string(),
                Alias::Command {
                    name: "invalid".to_string(),
                    cmd: Cmd::Invalid,
                },
            ),
        ]);

        let b = builtin_commands();
        assert_eq!(all, b);
    }
}
