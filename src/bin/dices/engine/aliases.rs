//! Module to load aliases from `$HOME/.config/dices/aliases`.
//!
//! Public API:
//!
//! ```no_run
//! # use std::path::PathBuf;
//! use dices_rs::engine::Engine;
//!
//! let e = Engine::new();
//! let aliases = e.load_aliases(Some(PathBuf::from("/some/location/aliases")))?;
//! ```
//! or et get only the default aliases:
//! ```no_run
//! # use std::path::PathBuf;
//! use dices_rs::engine::Engine;
//!
//! let e = Engine::new();
//! let aliases = e.load_aliases(None).unwrap();
//! ```
//!
//! File format:
//! ```text
//! # This is for adding a command
//! doom = "2D6"
//! # These replicate an existing one
//! mouv = move
//! dice = roll
//! ```

use std::fs;
use std::path::PathBuf;

use itertools::Itertools;
use log::{debug, trace};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, one_of, space0, space1},
    combinator::map,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::engine::{Command, Engine};

/// Parse a comment introduced by one of #, // and ! followed by a space
///
fn parse_comment(input: &str) -> IResult<&str, Command> {
    trace!("parse_comment");
    let ret_comment = |_s: &str| Command::Comment;
    let r = terminated(
        alt((tag("#"), tag("//"), tag("!"))),
        preceded(space1, is_not("\r\n")),
    );
    map(r, ret_comment)(input)
}

/// Parse a line, return a Command::Macro that will be interpreted above as existing (alias) or
/// new (macro)
///
fn parse_alias(input: &str) -> IResult<&str, Command> {
    trace!("parse_alias");
    let check = |(first, second): (&str, &str)| {
        trace!("{}", second);

        Command::Macro {
            name: first.to_string(),
            cmd: second.to_string(),
        }
    };
    let r = separated_pair(
        alpha1,
        delimited(space0, tag("="), space0),
        alt((parse_string, alpha1)),
    );
    map(r, check)(input)
}

/// Parse the new command
///
fn parse_string(input: &str) -> IResult<&str, &str> {
    trace!("parse_string");
    delimited(one_of("\"'"), is_not("\""), one_of("\"'"))(input)
}

impl Engine {
    /// Load aliases as a list of `Command`.
    ///
    pub fn with(&mut self, fname: Option<PathBuf>) -> &mut Self {
        trace!("with");

        // Always load builtins
        //
        let mut list = builtin_aliases();
        debug!("builtins = {:?}", list);

        let mut added = match fname {
            Some(fname) => {
                if fname.exists() {
                    trace!("Reading {:?} file...", fname);
                    let content = fs::read_to_string(fname).unwrap_or_else(|_| "".to_string());

                    // Get all from file
                    //
                    let added: Vec<Command> = content
                        .lines()
                        .filter_map(|line| {
                            let (_input, alias) = alt((parse_comment, parse_alias))(line).unwrap();
                            // Look at what we got
                            //
                            match alias {
                                // Check whether the "new" command points to a known command then
                                // it is an alias, not a new command
                                //
                                Command::Macro { name, cmd } => {
                                    // Do we have an alias to a known command?
                                    //
                                    if self.exist(&cmd) {
                                        Some(Command::Alias { name, cmd })
                                    } else {
                                        Some(Command::Macro { name, cmd })
                                    }
                                }
                                // Builtins are fine
                                //
                                Command::Builtin { .. } => Some(alias),
                                // Skip the rest
                                //
                                _ => None,
                            }
                        })
                        .collect();
                    added
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };

        // Merge our builtin aliases
        //
        list.append(&mut added);

        let list = list.into_iter().unique().collect::<Vec<Command>>();
        debug!("aliases={list:?}");
        trace!("{} aliases/macros added", list.len());

        self.merge(list)
    }
}
/// Define some builtin aliases
///
fn builtin_aliases() -> Vec<Command> {
    trace!("builtin_aliases");
    vec![
        // Dices of Doom(tm)
        //
        Command::Macro {
            name: "doom".to_string(),
            cmd: "dice 2D6".to_string(),
        },
        // Roll as Dice
        //
        Command::Alias {
            name: "roll".to_string(),
            cmd: "dice".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use crate::makepath;

    use super::*;

    #[test]
    fn test_parse_comment_sharp() {
        let c = parse_comment("# this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Command::Comment, c.1);
    }

    #[test]
    fn test_parse_comment_c() {
        let c = parse_comment("// this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Command::Comment, c.1);
    }

    #[test]
    fn test_parse_comment_exclamation() {
        let c = parse_comment("! this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Command::Comment, c.1);
    }

    #[test]
    fn test_parse_string() {
        let a = "\"this is a string\"";

        let r = parse_string(a);
        assert!(r.is_ok());
        let (_input, r) = r.unwrap();
        assert_eq!("this is a string", r);
    }

    #[test]
    fn test_load_aliases_with_file() {
        let fname: PathBuf = makepath!("testdata", "aliases");
        let al = vec![
            Command::Macro {
                name: "doom".to_string(),
                cmd: "dice 2D6".to_string(),
            },
            Command::Alias {
                name: "roll".to_string(),
                cmd: "dice".to_string(),
            },
            Command::Alias {
                name: "rulez".to_string(),
                cmd: "dice".to_string(),
            },
            Command::Macro {
                name: "move".to_string(),
                cmd: "dice 3D6 -9".to_string(),
            },
            Command::Macro {
                name: "mouv".to_string(),
                cmd: "move +7".to_string(),
            },
            Command::Alias {
                name: "quit".to_string(),
                cmd: "exit".to_string(),
            },
        ];

        let n = Engine::new().with(Some(fname));
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(al, n);
    }

    #[test]
    fn test_load_aliases_with_none() {
        let al = vec![
            Command::Macro {
                name: "doom".to_string(),
                cmd: "dice 2D6".to_string(),
            },
            Command::Alias {
                name: "roll".to_string(),
                cmd: "dice".to_string(),
            },
        ];

        let n = Engine::new().with(None);
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(al, n);
    }
}
