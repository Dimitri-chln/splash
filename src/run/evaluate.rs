use itertools::Itertools;

use crate::parse::{Atom, Expression, Identifier, Operator};

use super::{
    block::{self, BlockValue},
    builtin,
    context::Context,
    function::Function,
    value::Value,
    SplashRuntimeError,
};

pub type EvaluateResult<'a> = Result<Option<Value>, SplashRuntimeError<'a>>;

fn evaluate_atom<'a>(atom: &Atom<'a>, context: &mut Context<'a>) -> EvaluateResult<'a> {
    match atom {
        Atom::Literal(literal) => Ok(Some(Value::from(literal.clone()))),
        Atom::Identifier(identifier) => context.variable(identifier).map(Some),
    }
}

fn evaluate_operation<'a>(
    operator: &Operator,
    expressions: &[Expression<'a>],
    context: &mut Context<'a>,
) -> EvaluateResult<'a> {
    let values = expressions
        .iter()
        .map(|expression| evaluate(expression, context))
        .map_ok(|value| value.ok_or(SplashRuntimeError::NoValue))
        .flatten_ok()
        .collect::<Result<Vec<_>, _>>()?;

    match operator {
        Operator::Not => builtin::not(values[0].clone()),
        Operator::Plus => builtin::plus(values[0].clone(), values[1].clone()),
        Operator::Minus => builtin::minus(values[0].clone(), values[1].clone()),
        Operator::Times => builtin::times(values[0].clone(), values[1].clone()),
        Operator::Divide => builtin::divide(values[0].clone(), values[1].clone()),
        Operator::Modulo => builtin::modulo(values[0].clone(), values[1].clone()),
        Operator::Equal => builtin::equal(values[0].clone(), values[1].clone()),
        Operator::NotEqual => builtin::not_equal(values[0].clone(), values[1].clone()),
        Operator::GreaterThan => builtin::greater_than(values[0].clone(), values[1].clone()),
        Operator::GreaterOrEqual => builtin::greater_or_equal(values[0].clone(), values[1].clone()),
        Operator::LessThan => builtin::less_than(values[0].clone(), values[1].clone()),
        Operator::LessOrEqual => builtin::less_or_equal(values[0].clone(), values[1].clone()),
        Operator::And => builtin::and(values[0].clone(), values[1].clone()),
        Operator::Or => builtin::or(values[0].clone(), values[1].clone()),
    }
}

fn evaluate_function<'a>(
    identifier: &Identifier<'a>,
    parameters: &[Expression<'a>],
    context: &mut Context<'a>,
) -> EvaluateResult<'a> {
    let function = context.function(identifier).cloned()?;

    let parameters = parameters
        .iter()
        .map(|parameter| evaluate(parameter, context))
        .map_ok(|value| value.ok_or(SplashRuntimeError::NoValue))
        .flatten_ok()
        .collect::<Result<Vec<_>, _>>()?;

    match function {
        Function::BuiltIn(function) => function(&parameters),
        Function::Custom(arguments, body) => {
            if parameters.len() != arguments.len() {
                return Err(SplashRuntimeError::InvalidSignature(
                    *identifier,
                    arguments.len(),
                    parameters.len(),
                ));
            }

            context.child(|context| {
                for (i, parameter) in parameters.into_iter().enumerate() {
                    context.initialize_variable(arguments[i], parameter);
                }

                match block::run(&body, context)? {
                    BlockValue::Return(value) => Ok(value),
                    BlockValue::None => Ok(None),
                }
            })
        }
    }
}

pub fn evaluate<'a>(expression: &Expression<'a>, context: &mut Context<'a>) -> EvaluateResult<'a> {
    match expression {
        Expression::Atom(atom) => evaluate_atom(atom, context),
        Expression::Operation(operator, expressions) => {
            evaluate_operation(operator, expressions, context)
        }
        Expression::Function(identifier, parameters) => {
            evaluate_function(identifier, parameters, context)
        }
    }
}
