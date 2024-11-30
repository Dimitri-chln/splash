use std::iter::{Peekable, Rev};

use nom::{branch::alt, combinator::map, multi::many1, sequence::tuple, IResult, Parser};

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
    fn new(operator: Operator, operands: Vec<Operand<'a>>) -> Self {
        Self { operator, operands }
    }

    #[must_use]
    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    #[must_use]
    pub fn operands(&self) -> &[Operand<'a>] {
        &self.operands
    }
}

fn parse_operand(input: &str) -> IResult<&str, Operand, SplashParseError> {
    alt((
        map(
            trim(alt((
                parse_unary_operation,
                parentheses(trim(parse_multi_operation)),
            ))),
            Operand::Operation,
        ),
        map(trim(expression_no_operation), Operand::Expression),
    ))
    .parse(input)
}

fn parse_unary_operation(input: &str) -> IResult<&str, Operation, SplashParseError> {
    map(
        tuple((unary_operator, trim(parse_operand))),
        |(operator, operand)| Operation::new(operator, vec![operand]),
    )
    .parse(input)
}

fn transform_multi_operation<'a>(
    mut others: Peekable<Rev<std::vec::IntoIter<(Operand<'a>, Operator)>>>,
    right: Operand<'a>,
) -> Operation<'a> {
    match others.next() {
        Some((left, operator1)) => match others.peek() {
            Some((_, operator2)) => {
                if operator1.priority() < operator2.priority() {
                    Operation::new(
                        operator1,
                        vec![
                            Operand::Operation(transform_multi_operation(others, left)),
                            right,
                        ],
                    )
                } else {
                    let operation = Operation::new(operator1, vec![left, right]);
                    transform_multi_operation(others, Operand::Operation(operation))
                }
            }
            None => Operation::new(operator1, vec![left, right]),
        },
        None => match right {
            Operand::Operation(operation) => operation,
            Operand::Expression(_) => unreachable!("expressions shouldn't appear alone here"),
        },
    }
}

fn parse_multi_operation(input: &str) -> IResult<&str, Operation, SplashParseError> {
    alt((
        map(
            tuple((
                many1(tuple((trim(parse_operand), binary_operator))),
                trim(parse_operand),
            )),
            |(others, right)| transform_multi_operation(others.into_iter().rev().peekable(), right),
        ),
        trim(parse_unary_operation),
    ))
    .parse(input)
}

pub fn operation(input: &str) -> IResult<&str, Operation, SplashParseError> {
    parse_multi_operation(input)
}
