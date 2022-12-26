//! Module to load aliases from `$HOME/.config/dices/aliases`.
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

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, one_of, space0, space1},
    combinator::map,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::cmds::Cmd;

/// This describe all possibilities
///
#[derive(Debug, Eq, PartialEq)]
pub enum Alias {
    New { name: String, cmd: String },
    Cmd { name: String, cmd: Cmd },
    Comment,
}

/// Parse a comment introduced by one of #, // and ! followed by a space
///
fn parse_comment(input: &str) -> IResult<&str, Alias> {
    let ret_comment = |_s: &str| Alias::Comment;
    let r = terminated(
        alt((tag("#"), tag("//"), tag("!"))),
        preceded(space1, is_not("\r\n")),
    );
    map(r, ret_comment)(input)
}

/// Parse a line with either:
///
/// - command alias nom1 = nom2
/// - new command new = "3D4"
///
fn parse_alias(input: &str) -> IResult<&str, Alias> {
    let check = |(first, second): (&str, &str)| {
        dbg!(&second);
        let cmd = Cmd::from(second);

        // If the command is invalid, we have a new command, not an alias
        //
        match cmd {
            Cmd::Invalid => Alias::New {
                name: first.to_string(),
                cmd: second.to_string(),
            },
            _ => Alias::Cmd {
                name: first.to_string(),
                cmd,
            },
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
    delimited(one_of("\"'"), is_not(" \""), one_of("\"'"))(input)
}

pub fn load_aliases(fname: PathBuf) -> Result<Vec<Alias>> {
    let content = fs::read_to_string(fname)?;

    let list: Vec<Alias> = content
        .lines()
        .filter_map(|line| {
            let (_input, alias) = alt((parse_comment, parse_alias))(line).unwrap();
            // Skip comments
            //
            if alias != Alias::Comment {
                Some(alias)
            } else {
                None
            }
        })
        .collect();
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::makepath;

    #[test]
    fn test_parse_comment_sharp() {
        let c = parse_comment("# this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Alias::Comment, c.1);
    }

    #[test]
    fn test_parse_comment_c() {
        let c = parse_comment("// this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Alias::Comment, c.1);
    }

    #[test]
    fn test_parse_comment_exclamation() {
        let c = parse_comment("! this is a comment").unwrap();
        assert_eq!("", c.0);
        assert_eq!(Alias::Comment, c.1);
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
    fn test_load_aliases() {
        let fname: PathBuf = makepath!("testdata", "aliases");
        let al = vec![
            Alias::New {
                name: "doom".to_string(),
                cmd: "2D6".to_string(),
            },
            Alias::Cmd {
                name: "rulez".to_string(),
                cmd: Cmd::Dice,
            },
            Alias::Cmd {
                name: "roll".to_string(),
                cmd: Cmd::Dice,
            },
        ];

        let n = load_aliases(fname);
        assert!(n.is_ok());
        let n = n.unwrap();
        assert_eq!(al, n);
    }
}
