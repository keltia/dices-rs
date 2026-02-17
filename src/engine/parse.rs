use log::trace;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_till},
    character::complete::{alphanumeric1, char, one_of, space0},
    combinator::{map, map_res, opt},
    sequence::{delimited, preceded, separated_pair},
};
use std::string::ParseError;

use crate::engine::Command;

/// Parse a comment introduced by one of #, // and ! followed by a space
///
pub fn parse_comment(input: &str) -> IResult<&str, Command> {
    trace!("parse_comment");
    let r = preceded(
        alt((tag("#"), tag("//"), tag("!"))),
        opt(preceded(space0, is_not("\r\n"))),
    );
    map(r, |_| Command::Comment).parse(input)
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
        alphanumeric1,
        delimited(space0, tag("="), space0),
        alt((parse_string, alphanumeric1)),
    );
    map_res(r, check).parse(input)
}

/// Parse the new command
///
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    trace!("parse_string");
    let (input, quote) = one_of("\"'")(input)?;
    let (input, content) = take_till(|c| c == quote)(input)?;
    let (input, _) = char(quote)(input)?;
    Ok((input, content))
}
