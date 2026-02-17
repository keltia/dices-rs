//! This is the compiler part of the program that handles command parsing and compilation.
//!
//! The compiler is responsible for:
//! - Parsing raw input strings into commands
//! - Validating that commands exist
//! - Resolving macros and aliases recursively
//! - Producing executable Actions from the input
//!
//! The compilation process involves:
//! 1. Initial parsing of the command string
//! 2. Recursive resolution of macros/aliases up to MAX_RECUR depth
//! 3. Validation that the final command is executable
//! 4. Generation of an Action enum representing the command
//!

use std::collections::{HashMap, HashSet};

use eyre::{eyre, bail, Result};
use log::trace;
use nom::{character::complete::alphanumeric1, IResult};

use crate::engine::Command;

/// Action is more or less the result of the compilation done by `Compiler`
///
#[derive(Debug, PartialEq)]
pub enum Action {
    /// List aliases
    Aliases,
    /// This is the error
    Error(String),
    /// We need to execute a command
    Execute(Command, String),
    /// Get out
    Exit,
    /// List all commands
    List,
    /// List only macros
    Macros,
}

/// The Compiler handles parsing and compilation of command strings into executable Actions.
///
/// The Compiler maintains a map of valid commands and provides methods to:
/// - Parse raw input strings into commands
/// - Validate commands exist
/// - Resolve macros and aliases recursively
/// - Compile input into executable Actions
///
#[derive(Debug)]
pub struct Compiler<'a> {
    /// List of all available commands
    cmds: &'a HashMap<String, Command>,
}

impl<'a> Compiler<'a> {
    /// Max depth we allow for recursion
    ///
    pub const MAX_RECUR: usize = 10;

    /// Instantiate a new compiler with allowed commands
    ///
    pub fn new(cmds: &'a HashMap<String, Command>) -> Self {
        trace!("create compiler");
        Self { cmds }
    }

    /// We have the initial analysis of the input, resolve it into something we do know or
    /// something we can execute
    ///
    pub fn compile(&self, input: &str) -> Action {
        trace!("in compile({input})");

        let mut seen = HashSet::new();
        // Go directly into `recurse()`
        //
        let (input, cmd) = match self.recurse(input, None, &mut seen) {
            Ok((input, cmd)) => (input, cmd),
            Err(e) => return Action::Error(e.to_string()),
        };

        trace!("cmd={:?}", cmd);

        match cmd {
            Command::Exit => Action::Exit,
            Command::List => Action::List,
            Command::Aliases => Action::Aliases,
            Command::Macros => Action::Macros,

            // At this point these are not possible
            //
            Command::Macro { .. } => Action::Error("no macro".to_string()),
            Command::Alias { .. } => Action::Error("no alias".to_string()),

            // These can be executed directly
            //
            Command::Builtin { .. } => {
                // Identify and execute each command
                // Short one may be inserted here directly
                // otherwise put them in `engine/mod.rs`
                //
                trace!("builtin={:?}", cmd);
                Action::Execute(cmd, input)
            }
            _ => Action::Error("impossible command".to_string()),
        }
    }

    /// Parse then validate
    ///
    fn parse(&self, input: &str) -> Result<(String, Command)> {
        trace!("in compiler::parse({})", input);
        // Private fn
        //
        fn parse_keyword(input: &str) -> IResult<&str, &str> {
            alphanumeric1(input)
        }

        // Get command name
        //
        let (input, name) = match parse_keyword(input) {
            Ok((input, name)) => (input.to_owned(), name.to_owned()),
            Err(_) => return Err(eyre!("invalid command")),
        };

        trace!("name={name} with input={input}");

        // Validate that a given input does map to a `Command`
        //
        match self.cmds.get(&name) {
            Some(cmd) => {
                trace!("parse found {:?}", cmd);
                Ok((input, cmd.to_owned()))
            }
            None => return Err(eyre!("unknown command")),
        }
    }

    /// Try to reduce/compile `Macro` & `Alias` into a `Builtin` or special command
    ///
    /// This is a tail recursive function, might be turned into an iterative one at some point
    /// Not sure it is worth it.
    ///
    fn recurse(&self, input: &str, max: Option<usize>, seen: &mut HashSet<String>) -> Result<(String, Command)> {
        trace!("in compiler::recurse({max:?})={:?}", input);

        // Set default recursion max
        //
        let mut max = max.unwrap_or(Compiler::MAX_RECUR);

        let (input, command) = self.parse(input)?;

        // Cycle detection
        let name = match &command {
            Command::Alias { name, .. } | Command::Macro { name, .. } => name.clone(),
            _ => "".to_string(),
        };

        if !name.is_empty() {
            if seen.contains(&name) {
                bail!("recursion cycle detected for command: {}", name);
            }
            seen.insert(name);
        }

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
                cmd + input.as_str()
            }
            // XXX Need to recurse now but we must not lose any argument so append old input
            //
            Command::Macro { name, cmd } => {
                trace!("recurse=macro({})", name);
                cmd + input.as_str()
            }
            // These are builtin & special commands
            //
            Command::List | Command::Exit | Command::Aliases | Command::Macros => {
                trace!("list/exit, end");
                return Ok((input, command));
            }
            // Everything else is  an error here
            //
            _ => bail!("impossible in recurse"),
        };
        // Error out if too deep recursion
        //
        max -= 1;
        if max == 0 {
            return Err(eyre!("max recursion level reached for {}", input));
        }
        trace!("recurse(input)={input} max={max}");
        self.recurse(&input, Some(max), seen)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::engine::Engine;

    use super::*;

    #[rstest]
    #[case("exit", Action::Exit)]
    #[case("list", Action::List)]
    #[case("aliases", Action::Aliases)]
    #[case("macros", Action::Macros)]
    fn test_compile(#[case] input: &str, #[case] cmd: Action) {
        let n = Engine::new();
        let cc = Compiler::new(&n.cmds);
        assert_eq!(cmd, cc.compile(input))
    }
}
