//! This is the module which execute the different commands (builtin, alias, new, etc.).
//!

use std::fmt::Debug;

use anyhow::{anyhow, Result};
use log::{debug, error, trace};
use nom::{character::complete::space0, sequence::preceded};

use dices_rs::dice::{
    parse::{parse_open, parse_with_bonus},
    result::Res,
    Rollable,
};

use crate::commands::core::Cmd;

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

impl Command {
    /// Execute the given builtin command
    ///
    /// XXX Only builtin or aliases command can be executed here, no parsing context
    ///
    pub fn execute(self, input: &str) -> Result<Res> {
        match self {
            // Process builtins and aliases
            //
            Command::Builtin { cmd, .. } | Command::Alias { cmd, .. } => {
                trace!("builtin/alias");
                cmd.execute(input)
                // match cmd {
                //     Cmd::Dice => roll_from(input),
                //     Cmd::Open => roll_open(input),
                //     _ => Err(anyhow!("invalid command")),
                //}
            }
            _ => Err(anyhow!("should not happen")),
        }
    }
}
