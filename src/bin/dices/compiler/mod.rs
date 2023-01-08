use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use log::{debug, trace};
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
pub struct Compiler {
    cmds: HashMap<String, Command>,
}

impl Compiler {
    /// Max depth we allow for recusion
    ///
    pub const MAX_RECUR: usize = 5;

    /// Instanciate a new compiler with allowed commands
    ///
    pub fn new(cmds: &HashMap<String, Command>) -> Self {
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
            Ok((input, cmd)) => (input.to_string(), cmd),
            Err(_) => return Action::Error("unknown command".to_string()),
        };

        trace!("cmd={:?}", cmd);

        match cmd {
            // Shortcut to exit
            //
            Command::Exit => Action::Exit,

            // Shortcut to list
            //
            Command::List => Action::List,

            // Shortcut to exit
            //
            Command::Aliases => Action::Aliases,

            // Shortcut to list
            //
            Command::Macros => Action::Macros,

            // Re-enter the parser until be get to a Builtin
            //
            Command::Macro { cmd, .. } => Action::Error("no macro".to_string()),

            // Alias to something that may be a New or Alias
            //
            Command::Alias { cmd, .. } => Action::Error("no alias".to_string()),

            // These can be executed directly
            //
            Command::Builtin { .. } => {
                // Identify and execute each command
                // Short one may be inserted here directly
                // otherwise put them in `engine/mod.rs`
                //
                trace!("builtin={:?}", cmd);
                Action::Execute(cmd.clone(), input.to_string())
            }
            _ => return Action::Error("impossible command".to_string()),
        }
    }

    /// Parse then validate
    ///
    pub fn parse(&self, input: &str) -> Result<(String, Command)> {
        trace!("in compiler::parse({})", input);
        // Private fn
        //
        fn parse_keyword(input: &str) -> IResult<&str, &str> {
            alphanumeric1(input)
        }

        debug!("all={:?}", self.cmds);

        // Get command name
        //
        let (input, name) = match parse_keyword(input) {
            Ok((input, name)) => (input.to_owned(), name.to_owned()),
            Err(_) => return Err(anyhow!("invalid command")),
        };

        trace!("name={name} with input={input}");

        // Validate that a given input does map to a `Command`
        //
        match self.cmds.get(&name) {
            Some(cmd) => {
                trace!("parse found {:?}", cmd);
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
    pub fn recurse(&self, input: &str, max: Option<usize>) -> Result<(String, Command)> {
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
            return Err(anyhow!("max recursion level reached for {}", input));
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
