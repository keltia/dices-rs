//! List of builtin core commands (i.e. dice and not UI ones related ones.)
//!
//! Dice        Your regular dice
//! Open        Open-ended dice
//!
//! XXX If anyone add core commands, do not forget to document and test.

/// This describe the core commands in the rolling dice engine.
/// Everything above will be reduced (aka compiled) into executing
/// one of these.
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
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
}
