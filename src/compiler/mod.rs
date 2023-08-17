//! This is the compiler part of the program
//!
//! It does all the parsing, checking that the command we found does exist, resolve macros
//! and aliases and output our "compiled" code (aka `Action`) and the engine is supposed to
//! deal with the output.
//!

use std::collections::HashMap;

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

#[derive(Debug)]
/// Our compiler struct
///
pub struct Compiler {
    /// List of all available commands
    cmds: HashMap<String, Command>,
}

impl Compiler {
    /// Max depth we allow for recursion
    ///
    pub const MAX_RECUR: usize = 5;

    /// Instantiate a new compiler with allowed commands
    ///
    pub fn new(cmds: &HashMap<String, Command>) -> Self {
        trace!("create compiler with({:?})", cmds);
        Self { cmds: cmds.clone() }
    }

    /// We have the initial analysis of the input, resolve it into something we do know or
    /// something we can execute
    ///
    pub fn compile(&self, input: &str) -> Action {
        trace!("in compile({input})");

        // Go directly into `recurse()`
        //
        let (input, cmd) = match self.recurse(input, None) {
            Ok((input, cmd)) => (input, cmd),
            Err(_) => return Action::Error("unknown command".to_string()),
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
    fn recurse(&self, input: &str, max: Option<usize>) -> Result<(String, Command)> {
        trace!("in compiler::recurse({max:?})={:?}", input);

        // Set default recursion max
        //
        let mut max = max.unwrap_or(Compiler::MAX_RECUR);

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
        self.recurse(&input, Some(max))
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
