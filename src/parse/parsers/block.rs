use nom::{
    character::complete::char, combinator::map, multi::many0, sequence::delimited, IResult, Parser,
};

use crate::parse::{combinators::trim::trim, SplashParseError, Statement};

use super::statement::statement;

#[derive(Clone, Debug)]
pub struct Block<'a>(Vec<Statement<'a>>);

pub fn block(input: &str) -> IResult<&str, Block, SplashParseError> {
    map(
        delimited(char('{'), many0(trim(statement)), char('}')),
        Block,
    )
    .parse(input)
}

impl<'a> Block<'a> {
    #[must_use]
    pub fn statements(&self) -> &[Statement<'a>] {
        &self.0
    }
}
