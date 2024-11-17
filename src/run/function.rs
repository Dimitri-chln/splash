use crate::parse::{Block, Identifier};

use super::{evaluate::EvaluateResult, value::Value};

#[derive(Clone, Debug)]
pub enum Function<'a> {
    BuiltIn(fn(&[Value]) -> EvaluateResult<'a>),
    Custom(Vec<Identifier<'a>>, Block<'a>),
}
