use nom::{multi::many0, sequence::delimited, Parser};

use crate::parse::{parsers::comment::comment, SplashParseError};

use super::whitespace::whitespace;

pub fn trim<'a, O, F>(inner: F) -> impl Parser<&'a str, O, SplashParseError<'a>>
where
    F: Parser<&'a str, O, SplashParseError<'a>>,
{
    delimited(
        many0(whitespace(comment)),
        whitespace(inner),
        many0(whitespace(comment)),
    )
}
