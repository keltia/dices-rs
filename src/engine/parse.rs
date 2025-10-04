use log::trace;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, one_of, space0, space1},
    combinator::map_res,
    sequence::{delimited, preceded, separated_pair, terminated},
};
use std::string::ParseError;

use crate::engine::Command;

/// Parse a comment introduced by one of #, // and ! followed by a space
///
pub fn parse_comment(input: &str) -> IResult<&str, Command> {
    trace!("parse_comment");
    let ret_comment = |_s: &str| -> Result<Command, ParseError> { Ok(Command::Comment) };
    let r = terminated(
        alt((tag("#"), tag("//"), tag("!"))),
        preceded(space1, is_not("\r\n")),
    );
    map_res(r, ret_comment).parse(input)
}

/// Parse a line, return a Command::Macro that will be interpreted above as existing (alias) or
/// new (macro)
///
pub fn parse_alias(input: &str) -> IResult<&str, Command> {
    trace!("parse_alias");
    let check = |(first, second): (&str, &str)| -> Result<Command, ParseError> {
        trace!("{}", second);

        Ok(Command::Macro {
            name: first.to_string(),
            cmd: second.to_string(),
        })
    };
    let r = separated_pair(
        alpha1,
        delimited(space0, tag("="), space0),
        alt((parse_string, alpha1)),
    );
    map_res(r, check).parse(input)
}

/// Parse the new command
///
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    trace!("parse_string");
    delimited(one_of("\"'"), is_not("\""), one_of("\"'")).parse(input)
}
