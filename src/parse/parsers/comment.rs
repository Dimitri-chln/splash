use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::value,
    sequence::tuple,
    IResult, Parser,
};

use crate::parse::SplashParseError;

pub fn comment(input: &str) -> IResult<&str, (), SplashParseError> {
    value(
        (), // Output is thrown away.
        alt((
            tuple((tag("//"), take_until("\n"), tag("\n"))),
            tuple((tag("/*"), take_until("*/"), tag("*/"))),
        )),
    )
    .parse(input)
}
