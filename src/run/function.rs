use crate::parse::{Block, Identifier};

use super::{evaluate::Result, value::Value};

#[derive(Clone, Debug)]
pub enum Function<'a> {
    BuiltIn(fn(&[Value]) -> Result<'a>),
    Custom(Vec<Identifier<'a>>, Block<'a>),
}
