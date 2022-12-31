//! This is the module which execute the different commands (builtin, alias, new, etc.).
//!

use std::fmt::Debug;

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
