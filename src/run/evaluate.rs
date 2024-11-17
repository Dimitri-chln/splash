use itertools::Itertools;

use crate::parse::{Atom, BinaryOperator, Expression, Identifier, UnaryOperator};

use super::{
    block::{self, BlockValue},
    builtin::{
        and, divide, equal, greater_or_equal, greater_than, less_or_equal, less_than, minus,
        modulo, not, not_equal, or, plus, times,
    },
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

fn evaluate_unary_operation<'a>(
    operator: &UnaryOperator,
    expression: &Expression<'a>,
    context: &mut Context<'a>,
) -> EvaluateResult<'a> {
    let value = evaluate(expression, context)?.ok_or(SplashRuntimeError::NoValue)?;

    match operator {
        UnaryOperator::Not => not(value),
    }
}

fn evaluate_binary_operation<'a>(
    left: &Expression<'a>,
    operator: &BinaryOperator,
    right: &Expression<'a>,
    context: &mut Context<'a>,
) -> EvaluateResult<'a> {
    let left = evaluate(left, context)?.ok_or(SplashRuntimeError::NoValue)?;
    let right = evaluate(right, context)?.ok_or(SplashRuntimeError::NoValue)?;

    match operator {
        BinaryOperator::Plus => plus(left, right),
        BinaryOperator::Minus => minus(left, right),
        BinaryOperator::Times => times(left, right),
        BinaryOperator::Divide => divide(left, right),
        BinaryOperator::Modulo => modulo(left, right),
        BinaryOperator::Equal => equal(left, right),
        BinaryOperator::NotEqual => not_equal(left, right),
        BinaryOperator::GreaterThan => greater_than(left, right),
        BinaryOperator::GreaterOrEqual => greater_or_equal(left, right),
        BinaryOperator::LessThan => less_than(left, right),
        BinaryOperator::LessOrEqual => less_or_equal(left, right),
        BinaryOperator::And => and(left, right),
        BinaryOperator::Or => or(left, right),
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
                    context.set_variable(arguments[i], parameter);
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
        Expression::UnaryOperation(operator, expression) => {
            evaluate_unary_operation(operator, expression, context)
        }
        Expression::BinaryOperation(left, operator, right) => {
            evaluate_binary_operation(left, operator, right, context)
        }
        Expression::Function(identifier, parameters) => {
            evaluate_function(identifier, parameters, context)
        }
    }
}
