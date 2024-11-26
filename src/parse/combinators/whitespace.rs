use nom::{character::complete::multispace0, error::ParseError, sequence::delimited, Parser};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn whitespace<'a, O, E, F>(inner: F) -> impl Parser<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
