//! Module to load aliases from `$HOME/.config/dices/aliases`.
//!
//! You can define macros or aliases in there, they will be resolved at run-time
//! by the compiler.
//!
//! Public API:
//!
//! ```no_run
//! # use std::path::PathBuf;
//! use dices_rs::Engine;
//!
//! let e = Engine::new().with(Some(PathBuf::from("/some/location/aliases")));
//! ```
//! or et get only the default aliases:
//! ```no_run
//! # use std::path::PathBuf;
//! use dices_rs::Engine;
//!
//! let e = Engine::new().with(None);
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
use nom::Parser;
use nom::branch::alt;

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

                    // Get all from the file
                    //
                    let added: Vec<Command> = content
                        .lines()
                        .filter_map(|line| {
                            let line = line.trim();
                            if line.is_empty() {
                                return None;
                            }

                            let parsed = alt((parse_comment, parse_alias)).parse(line);
                            let (rest, alias) = match parsed {
                                Ok((rest, alias)) => (rest, alias),
                                Err(e) => {
                                    debug!("Skipping invalid alias line '{line}': {e:?}");
                                    return None;
                                }
                            };
                            if !rest.trim().is_empty() {
                                debug!("Skipping alias line with trailing garbage '{line}'");
                                return None;
                            }

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
    use super::*;
    use crate::engine::parse::parse_string;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_parse_comment_sharp() {
        let c = parse_comment("# this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Command::Comment, c.1);
    }

    #[test]
    fn test_parse_comment_sharp_no_space() {
        let c = parse_comment("#comment").unwrap();
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
        let fname = Path::new("testdata").join("aliases");
        let all = HashMap::<String, Command>::from([
            (
                "doom".to_string(),
                Command::Macro {
                    name: "doom".to_string(),
                    cmd: "dice 2D6".to_string(),
                },
            ),
            (
                "roll".to_string(),
                Command::Alias {
                    name: "roll".to_string(),
                    cmd: "dice".to_string(),
                },
            ),
            (
                "rulez".to_string(),
                Command::Alias {
                    name: "rulez".to_string(),
                    cmd: "dice".to_string(),
                },
            ),
            (
                "move".to_string(),
                Command::Macro {
                    name: "move".to_string(),
                    cmd: "dice 3D6 -9".to_string(),
                },
            ),
            (
                "mouv".to_string(),
                Command::Macro {
                    name: "mouv".to_string(),
                    cmd: "move +7".to_string(),
                },
            ),
            (
                "quit".to_string(),
                Command::Alias {
                    name: "quit".to_string(),
                    cmd: "exit".to_string(),
                },
            ),
            (
                "llist".to_string(),
                Command::Alias {
                    name: "llist".to_string(),
                    cmd: "list".to_string(),
                },
            ),
            ("aliases".to_string(), Command::Aliases),
            ("exit".to_string(), Command::Exit),
            ("macros".to_string(), Command::Macros),
        ]);

        let n = Engine::new().with(Some(fname)).build();

        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }

    #[test]
    fn test_load_aliases_with_none() {
        let all = HashMap::<String, Command>::from([
            (
                "roll".to_string(),
                Command::Alias {
                    name: "roll".to_string(),
                    cmd: "dice".to_string(),
                },
            ),
            ("exit".to_string(), Command::Exit),
            (
                "doom".to_string(),
                Command::Macro {
                    name: "doom".to_string(),
                    cmd: "dice 2D6".to_string(),
                },
            ),
            ("aliases".to_string(), Command::Aliases),
            ("macros".to_string(), Command::Macros),
        ]);

        let n = Engine::new().with(None).build();
        eprintln!("{:?}", n.cmds);

        all.into_iter().for_each(|(name, cmd)| {
            assert!(n.cmds.contains_key(&name));
            assert_eq!(&cmd, n.cmds.get(&name).unwrap());
        });
    }
}
