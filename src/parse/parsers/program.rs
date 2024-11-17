use nom::{combinator::all_consuming, IResult, Parser};

use crate::parse::{Block, SplashParseError};

use super::block::block;

pub type Program<'a> = Block<'a>;

pub fn program(input: &str) -> IResult<&str, Program, SplashParseError> {
    all_consuming(block).parse(input)
}
