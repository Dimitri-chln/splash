use itertools::Itertools;

use crate::parse::Expression;

use super::{context::Context, evaluate::evaluate, value::Value, SplashRuntimeError};

pub fn evaluate_predicate<'a>(
    expression: &Expression<'a>,
    context: &mut Context<'a>,
) -> Result<bool, SplashRuntimeError<'a>> {
    let value = evaluate(expression, context)?;

    match value {
        Some(Value::Boolean(boolean)) => Ok(boolean),
        Some(value) => Err(SplashRuntimeError::InvalidPredicate(value)),
        None => Err(SplashRuntimeError::NoValue),
    }
}

pub fn evaluate_values<'a>(
    expressions: &[Expression<'a>],
    context: &mut Context<'a>,
) -> Result<Vec<Value>, SplashRuntimeError<'a>> {
    expressions
        .iter()
        .map(|expression| evaluate(expression, context))
        .map_ok(|value| value.ok_or(SplashRuntimeError::NoValue))
        .flatten_ok()
        .collect::<Result<Vec<_>, _>>()
}
