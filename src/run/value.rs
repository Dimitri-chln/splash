use std::fmt::Display;

use crate::parse::Literal;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    List(Vec<Value>),
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Number(number) => Value::Number(number),
            Literal::Boolean(boolean) => Value::Boolean(boolean),
            Literal::String(string) => Value::String(string),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => number.fmt(f),
            Self::Boolean(boolean) => boolean.fmt(f),
            Self::String(string) => string.fmt(f),
            Self::List(list) => write!(
                f,
                "{}",
                format_args!(
                    "[{}]",
                    list.iter()
                        .map(|value| format!("{value}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            ),
        }
    }
}
