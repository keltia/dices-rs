use log::trace;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, one_of, space0, space1},
    combinator::map,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use crate::engine::Command;

/// Parse a comment introduced by one of #, // and ! followed by a space
///
pub fn parse_comment(input: &str) -> IResult<&str, Command> {
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
pub fn parse_alias(input: &str) -> IResult<&str, Command> {
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
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    trace!("parse_string");
    delimited(one_of("\"'"), is_not("\""), one_of("\"'"))(input)
}

