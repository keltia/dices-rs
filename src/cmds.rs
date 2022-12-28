use dices_rs::dice::{
    parse::{parse_open, parse_with_bonus},
    result::Res,
    Rollable,
};
use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::{anyhow, Result};
use log::{debug, error, trace};
use nom::{character::complete::space0, sequence::preceded};

use crate::core::Cmd;

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
}

impl Command {
    /// Execute the given command
    ///
    pub fn execute(self, input: &str) -> Result<Res> {
        match self {
            // Process builtins and aliases
            //
            Command::Builtin { cmd, .. } | Command::Alias { cmd, .. } => {
                trace!("builtin/alias");
                match cmd {
                    Cmd::Dice => roll_from(input),
                    Cmd::Open => roll_open(input),
                    _ => Err(anyhow!("invalid command")),
                }
            }
            // Process new commands:
            // Here we need to re-enter the parser as if it was typed in
            //
            Command::New { .. } => Ok(Res::new()),
            _ => Err(anyhow!("should not happen")),
        }
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
