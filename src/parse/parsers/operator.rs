use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value,
    error::ParseError, IResult, Parser,
};

#[derive(Clone, Debug)]
pub enum Operator {
    // Unary
    Not,
    // Binary
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    GreaterOrEqual,
    GreaterThan,
    LessOrEqual,
    LessThan,
    And,
    Or,
}

pub fn unary_operator<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Operator, E> {
    value(Operator::Not, char('!')).parse(input)
}

pub fn binary_operator<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Operator, E> {
    alt((
        value(Operator::Plus, char('+')),
        value(Operator::Minus, char('-')),
        value(Operator::Times, char('*')),
        value(Operator::Divide, char('/')),
        value(Operator::Modulo, char('%')),
        value(Operator::Equal, tag("==")),
        value(Operator::NotEqual, tag("!=")),
        value(Operator::GreaterOrEqual, tag(">=")),
        value(Operator::GreaterThan, char('>')),
        value(Operator::LessOrEqual, tag("<=")),
        value(Operator::LessThan, char('<')),
        value(Operator::And, tag("&&")),
        value(Operator::Or, tag("||")),
    ))
    .parse(input)
}
