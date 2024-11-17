use crate::parse::Literal;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
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
