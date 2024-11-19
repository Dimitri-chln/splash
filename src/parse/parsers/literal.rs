use nom::{
    branch::alt, bytes::complete::tag, combinator::map, number::complete::double, IResult, Parser,
};

use crate::parse::SplashParseError;

use super::string::string;

#[derive(Clone, Debug)]
pub enum Literal {
    Number(f64),
    Boolean(bool),
    String(String),
}

fn parse_number(input: &str) -> IResult<&str, Literal, SplashParseError> {
    map(double, Literal::Number).parse(input)
}

fn parse_boolean(input: &str) -> IResult<&str, Literal, SplashParseError> {
    alt((
        map(tag("true"), |_| Literal::Boolean(true)),
        map(tag("false"), |_| Literal::Boolean(false)),
    ))
    .parse(input)
}

fn parse_string(input: &str) -> IResult<&str, Literal, SplashParseError> {
    map(string, Literal::String).parse(input)
}

pub fn literal(input: &str) -> IResult<&str, Literal, SplashParseError> {
    alt((parse_number, parse_boolean, parse_string)).parse(input)
}
