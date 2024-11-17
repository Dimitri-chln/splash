use nom::{
    branch::alt, bytes::complete::tag, combinator::map, number::complete::double, IResult, Parser,
};

use crate::parse::SplashParseError;

use super::string::string;

#[derive(Clone, Debug)]
pub enum Literal {
    // Integer(i64),
    Number(f64),
    Boolean(bool),
    String(String),
}

// fn parse_integer(input: &str) -> IResult<&str, Literal> {
//     map(
//         tuple((opt(char('-')), map_res(digit1, str::parse::<i64>))),
//         |(sign, integer)| match sign {
//             Some(_) => Literal::Integer(-integer),
//             None => Literal::Integer(integer),
//         },
//     )
//     .parse(input)
// }

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
    alt((
        //parse_integer,
        parse_number,
        parse_boolean,
        parse_string,
    ))
    .parse(input)
}
