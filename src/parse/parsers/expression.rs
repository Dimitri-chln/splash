use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult, Parser,
};

use crate::parse::{combinators::trim::trim, SplashParseError};

use super::{
    atom::{atom, Atom},
    identifier::{identifier, Identifier},
    operator::{binary_operator, unary_operator, Operator},
};

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Atom(Atom<'a>),
    Operation(Operator, Vec<Expression<'a>>),
    Function(Identifier<'a>, Vec<Expression<'a>>),
}

fn parse_atom(input: &str) -> IResult<&str, Expression, SplashParseError> {
    map(trim(atom), Expression::Atom).parse(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expression, SplashParseError> {
    alt((
        map(
            tuple((unary_operator, trim(expression))),
            |(operator, expression)| Expression::Operation(operator, vec![expression]),
        ),
        map(
            delimited(
                char('('),
                tuple((trim(expression), binary_operator, trim(expression))),
                char(')'),
            ),
            |(left, operator, right)| Expression::Operation(operator, vec![left, right]),
        ),
    ))
    .parse(input)
}

fn parse_function(input: &str) -> IResult<&str, Expression, SplashParseError> {
    map(
        tuple((
            identifier,
            delimited(
                char('('),
                separated_list0(char(','), trim(expression)),
                char(')'),
            ),
        )),
        |(name, parameters)| Expression::Function(name, parameters),
    )
    .parse(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression, SplashParseError> {
    alt((parse_function, parse_operation, parse_atom)).parse(input)
}
