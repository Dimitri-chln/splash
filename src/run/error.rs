use std::{error::Error, fmt::Display};

use crate::parse::Identifier;

use super::value::Value;

#[derive(Debug)]
pub enum SplashRuntimeError<'a> {
    NotDefined(Identifier<'a>),
    NoValue,
    InvalidType(Identifier<'a>),
    InvalidSignature(Identifier<'a>, usize, usize),
    InvalidSignatureType(Identifier<'a>, Vec<Value>),
    InvalidPredicate(Value),
    DivisionByZero,
}

impl Display for SplashRuntimeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotDefined(identifier) => format!("'{identifier}' is not defined."),
                Self::NoValue => String::from("Expected a value, but the expression returned nothing."),
                Self::InvalidType(identifier) => format!("'{identifier}' is of the wrong type."),
				Self::InvalidSignature(identifier, expected, actual) => format!("Function '{identifier}' takes {expected} arguments, but {actual} were provided."),
				Self::InvalidSignatureType(identifier, parameters) => format!("Incorrect arguments were given to the function '{identifier}'. Received ({parameters:?})."),
				Self::InvalidPredicate(predicate) => format!("Predicate must be a boolean. Received '{predicate:?}'."),
				Self::DivisionByZero => String::from("Cannot divide by zero.")
            }
        )
    }
}

impl Error for SplashRuntimeError<'_> {}
