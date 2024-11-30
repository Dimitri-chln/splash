use nom::{branch::alt, combinator::map, sequence::tuple, IResult, Parser};

use crate::parse::{
    combinators::{parentheses::parentheses, trim::trim},
    Expression, Operator, SplashParseError,
};

use super::{
    expression::expression_no_operation,
    operator::{binary_operator, unary_operator},
};

#[derive(Clone, Debug)]
pub enum Operand<'a> {
    Operation(Operation<'a>),
    Expression(Expression<'a>),
}

#[derive(Clone, Debug)]
pub struct Operation<'a> {
    operator: Operator,
    operands: Vec<Operand<'a>>,
}

impl<'a> Operation<'a> {
    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn operands(&self) -> &[Operand<'a>] {
        &self.operands
    }
}

fn operand(input: &str) -> IResult<&str, Operand, SplashParseError> {
    alt((
        map(trim(parentheses(operation)), Operand::Operation),
        map(trim(expression_no_operation), Operand::Expression),
    ))
    .parse(input)
}

fn parse_unary_operation(input: &str) -> IResult<&str, Operation, SplashParseError> {
    map(tuple((unary_operator, operand)), |(operator, operand)| {
        Operation {
            operator,
            operands: vec![operand],
        }
    })
    .parse(input)
}

fn parse_binary_operation(input: &str) -> IResult<&str, Operation, SplashParseError> {
    map(
        tuple((trim(operand), binary_operator, trim(operand))),
        |(left, operator, right)| Operation {
            operator,
            operands: vec![left, right],
        },
    )
    .parse(input)
}

pub fn operation(input: &str) -> IResult<&str, Operation, SplashParseError> {
    alt((parse_unary_operation, parse_binary_operation)).parse(input)
}
