use nom::{character::complete::char, error::ParseError, sequence::delimited, Parser};

pub fn parentheses<'a, O, E, F>(inner: F) -> impl Parser<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(char('('), inner, char(')'))
}
