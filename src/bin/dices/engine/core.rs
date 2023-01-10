//! List of builtin core commands (i.e. dice and not UI ones related ones.)
//!
//! Dice        Your regular dice
//! Open        Open-ended dice
//!
//! XXX If anyone add core commands, do not forget to document and test.

use anyhow::{anyhow, Result};
use log::{debug, error, trace};
use nom::{character::complete::space0, sequence::preceded};
use serde::{Deserialize, Serialize};

use dices_rs::dice::{
    parse::{parse_open, parse_with_bonus},
    result::Res,
    Rollable,
};

/// This describe the core commands in the rolling dice engine.
/// Everything above will be reduced (aka compiled) into executing
/// one of these.
///
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Cmd {
    /// Roll of dices
    Dice,
    /// Invalid command
    Invalid,
    /// Roll an open dice
    Open,
}

impl From<&str> for Cmd {
    /// Return the command associated with the keyword (excluding aliases)
    ///
    fn from(value: &str) -> Self {
        match value {
            "dice" => Cmd::Dice,
            "open" => Cmd::Open,
            _ => Cmd::Invalid,
        }
    }
}

impl Cmd {
    pub fn execute(&self, input: &str) -> Result<Res> {
        trace!("cmd::execute");
        let r = match self {
            Cmd::Dice => preceded(space0, parse_with_bonus)(input),
            Cmd::Open => preceded(space0, parse_open)(input),
            _ => return Err(anyhow!("invalid Cmd")),
        };
        let ds = match r {
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
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("dice", Cmd::Dice)]
    #[case("open", Cmd::Open)]
    #[case("doce", Cmd::Invalid)]
    #[case("doom", Cmd::Invalid)]
    #[case("whatever", Cmd::Invalid)]
    fn test_cmd_from(#[case] input: &str, #[case] cmd: Cmd) {
        assert_eq!(cmd, Cmd::from(input))
    }

    #[rstest]
    #[case("dice", "D6", Cmd::Dice)]
    #[case("dice", "2d4", Cmd::Dice)]
    #[case("open", "d4", Cmd::Open)]
    #[case("open", "D4", Cmd::Open)]
    fn test_cmd_execute(#[case] cmd: &str, #[case] arg: &str, #[case] ds: Cmd) {
        let d = Cmd::from(cmd);
        assert_eq!(ds, d);
        let res = d.execute(arg);
        assert!(res.is_ok());
    }
}
