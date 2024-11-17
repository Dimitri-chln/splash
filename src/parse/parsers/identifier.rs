use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
    IResult, Parser,
};

use crate::parse::SplashParseError;

pub type Identifier<'a> = &'a str;

pub fn identifier(input: &str) -> IResult<&str, Identifier<'_>, SplashParseError> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}
