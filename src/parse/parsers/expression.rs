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
    operation::{operation, Operation},
};

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Atom(Atom<'a>),
    Operation(Operation<'a>),
    Function(Identifier<'a>, Vec<Expression<'a>>),
    List(Vec<Expression<'a>>),
    Index(Identifier<'a>, Box<Expression<'a>>),
}

fn parse_atom(input: &str) -> IResult<&str, Expression, SplashParseError> {
    map(trim(atom), Expression::Atom).parse(input)
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

fn parse_list(input: &str) -> IResult<&str, Expression, SplashParseError> {
    map(
        delimited(
            char('['),
            separated_list0(char(','), trim(expression)),
            char(']'),
        ),
        Expression::List,
    )
    .parse(input)
}

fn parse_index(input: &str) -> IResult<&str, Expression, SplashParseError> {
    map(
        tuple((
            identifier,
            delimited(char('['), trim(expression), char(']')),
        )),
        |(identifier, index)| Expression::Index(identifier, Box::new(index)),
    )
    .parse(input)
}

pub fn expression_no_operation(input: &str) -> IResult<&str, Expression, SplashParseError> {
    alt((parse_index, parse_list, parse_function, parse_atom)).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression, SplashParseError> {
    alt((
        map(operation, Expression::Operation),
        expression_no_operation,
    ))
    .parse(input)
}
