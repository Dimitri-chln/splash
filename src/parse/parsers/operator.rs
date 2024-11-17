use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value,
    error::ParseError, IResult, Parser,
};

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    And,
    Or,
}

pub fn unary_operator<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, UnaryOperator, E> {
    value(UnaryOperator::Not, char('!')).parse(input)
}

pub fn binary_operator<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, BinaryOperator, E> {
    alt((
        value(BinaryOperator::Plus, char('+')),
        value(BinaryOperator::Minus, char('-')),
        value(BinaryOperator::Times, char('*')),
        value(BinaryOperator::Divide, char('/')),
        value(BinaryOperator::Modulo, char('%')),
        value(BinaryOperator::Equal, tag("==")),
        value(BinaryOperator::NotEqual, tag("!=")),
        value(BinaryOperator::GreaterThan, char('>')),
        value(BinaryOperator::GreaterOrEqual, tag(">=")),
        value(BinaryOperator::LessThan, char('<')),
        value(BinaryOperator::LessOrEqual, tag("<=")),
        value(BinaryOperator::And, tag("&&")),
        value(BinaryOperator::Or, tag("||")),
    ))
    .parse(input)
}
