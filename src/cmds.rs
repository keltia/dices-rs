use dices_rs::dice::{
    parse::{parse_open, parse_with_bonus},
    result::Res,
    Rollable,
};

use anyhow::{anyhow, Result};
use log::{debug, error};
use nom::{
    character::complete::{alpha1, space0},
    combinator::map,
    sequence::preceded,
    IResult,
};

/// List of existing commands
///
#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    Dice,
    Doom,
    Exit,
    Invalid,
    Move,
    Open,
}

impl From<&str> for Cmd {
    /// Return the command associated with the keyword (excluding aliases)
    ///
    fn from(value: &str) -> Self {
        match value {
            "dice" => Cmd::Dice,
            "doom" => Cmd::Doom,
            "exit" => Cmd::Exit,
            "move" => Cmd::Move,
            "open" => Cmd::Open,
            _ => Cmd::Invalid,
        }
    }
}

/// Parse a keyword, return the operation
///
pub fn parse_keyword(input: &str) -> IResult<&str, Cmd> {
    let get_op = |s: &str| match s.to_ascii_lowercase().as_str() {
        "doom" => Cmd::Doom,
        "dice" => Cmd::Dice,
        "mouv" => Cmd::Move,
        "move" => Cmd::Move,
        "open" => Cmd::Open,
        "roll" => Cmd::Dice,
        "exit" => Cmd::Exit,
        _ => Cmd::Invalid,
    };
    let r = alpha1;
    map(r, get_op)(input)
}

/// Generic roller
///
pub fn roll_from(input: &str) -> Result<Res> {
    let ds = match preceded(space0, parse_with_bonus)(input) {
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

/// Generic open dice roller
///
pub fn roll_open(input: &str) -> Result<Res> {
    let d = match preceded(space0, parse_open)(input) {
        Ok((_input, d)) => {
            debug!("{:?}", d);
            d
        }
        Err(e) => {
            error!("{:?}", e.to_string());
            return Err(anyhow!("error parsing input"));
        }
    };
    Ok(d.roll())
}
