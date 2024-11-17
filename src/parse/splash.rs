use nom::{IResult, Parser};

use super::{
    parsers::program::{program, Program},
    SplashParseError,
};

pub struct SplashParser;

impl<'a> Parser<&'a str, Program<'a>, SplashParseError<'a>> for SplashParser {
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, Program<'a>, SplashParseError<'a>> {
        program(input)
    }
}
