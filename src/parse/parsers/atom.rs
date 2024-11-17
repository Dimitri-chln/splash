use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::parse::SplashParseError;

use super::{
    identifier::{identifier, Identifier},
    literal::{literal, Literal},
};

#[derive(Clone, Debug)]
pub enum Atom<'a> {
    Literal(Literal),
    Identifier(Identifier<'a>),
}

pub fn atom(input: &str) -> IResult<&str, Atom, SplashParseError> {
    alt((
        map(literal, Atom::Literal),
        map(identifier, Atom::Identifier),
    ))
    .parse(input)
}
