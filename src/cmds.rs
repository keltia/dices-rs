use nom::Err::Error;
use nom::{
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, opt},
    sequence::pair,
    IResult,
};

#[derive(Debug)]
pub enum Cmd {
    Simple(Op),
    WithArgs(Op, String),
}

#[derive(Debug)]
pub enum Op {
    Exit,
    Invalid,
    Roll,
}

/// Parse a keyword, return the operation
///
fn parse_keyword(input: &str) -> IResult<&str, Op> {
    let get_op = |s: &str| match s.to_ascii_lowercase().as_str() {
        "dice" => Op::Roll,
        "exit" => Op::Exit,
        _ => Op::Invalid,
    };
    let r = alpha1;
    map(r, get_op)(input)
}

/// Parse a keyword and optional arguments, return the command & args
///
pub fn parse_line(input: &str) -> IResult<&str, Cmd> {
    let get_args = |(op, args): (Op, Option<&str>)| match op {
        Op::Roll => match args {
            Some(args) => Cmd::WithArgs(op, args.to_owned()),
            None => Err(Error("need arguments")),
        },
        _ => Cmd::Simple(op),
    };
    let r = pair(parse_keyword, opt(alphanumeric1));
    map(r, get_args)(input)
}
