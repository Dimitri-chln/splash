use crate::parse::Literal;

#[derive(Clone, Debug)]
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

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Number(number) => number.to_string(),
            Self::Boolean(boolean) => boolean.to_string(),
            Self::String(string) => string.to_string(),
            Self::List(list) => format!(
                "[{}]",
                list.iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
