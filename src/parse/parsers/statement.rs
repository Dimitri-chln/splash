use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use crate::parse::{combinators::trim::trim, SplashParseError};

use super::{
    block::{block, Block},
    expression::{expression, Expression},
    identifier::{identifier, Identifier},
    keyword::{keyword, Keyword},
};

#[derive(Clone, Debug)]
pub enum Statement<'a> {
    Simple(Expression<'a>),
    Block(Block<'a>),
    Assignment(Identifier<'a>, Expression<'a>),
    If(Expression<'a>, Block<'a>),
    IfElse(Expression<'a>, Block<'a>, Block<'a>),
    While(Expression<'a>, Block<'a>),
    Return(Option<Expression<'a>>),
    Definition(Identifier<'a>, Vec<Identifier<'a>>, Block<'a>),
}

fn parse_simple(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(trim(expression), Statement::Simple).parse(input)
}

fn parse_block(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(block, Statement::Block).parse(input)
}

fn parse_assignment(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(
        tuple((terminated(trim(identifier), char('=')), trim(expression))),
        |(identifier, expression)| Statement::Assignment(identifier, expression),
    )
    .parse(input)
}

fn parse_if(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(
        tuple((
            preceded(keyword(Keyword::If), trim(expression)),
            trim(block),
        )),
        |(predicate, then)| Statement::If(predicate, then),
    )
    .parse(input)
}

fn parse_if_else(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(
        tuple((
            preceded(keyword(Keyword::If), trim(expression)),
            trim(block),
            preceded(keyword(Keyword::Else), trim(block)),
        )),
        |(predicate, then, otherwise)| Statement::IfElse(predicate, then, otherwise),
    )
    .parse(input)
}

fn parse_while(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(
        tuple((
            preceded(keyword(Keyword::While), trim(expression)),
            trim(block),
        )),
        |(predicate, body)| Statement::While(predicate, body),
    )
    .parse(input)
}

fn parse_return(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(
        preceded(keyword(Keyword::Return), opt(trim(expression))),
        Statement::Return,
    )
    .parse(input)
}

fn parse_definition(input: &str) -> IResult<&str, Statement, SplashParseError> {
    map(
        tuple((
            preceded(keyword(Keyword::Fn).and(char(' ')), trim(identifier)),
            delimited(
                char('('),
                separated_list0(char(','), trim(identifier)),
                char(')'),
            ),
            trim(block),
        )),
        |(identifier, arguments, body)| Statement::Definition(identifier, arguments, body),
    )
    .parse(input)
}

pub fn statement(input: &str) -> IResult<&str, Statement, SplashParseError> {
    terminated(
        alt((
            parse_definition,
            parse_return,
            parse_while,
            parse_if_else,
            parse_if,
            parse_assignment,
            parse_block,
            parse_simple,
        )),
        opt(char(';')),
    )
    .parse(input)
}
