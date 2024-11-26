use crate::parse::{Atom, Expression, Identifier, Operator};

use super::{
    block::{self, BlockValue},
    builtin,
    context::Context,
    function::Function,
    utils::evaluate_values,
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
    operands: &[Expression<'a>],
    context: &mut Context<'a>,
) -> EvaluateResult<'a> {
    let values = evaluate_values(operands, context)?;

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
    let parameters = evaluate_values(parameters, context)?;

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

fn evaluate_list<'a>(elements: &[Expression<'a>], context: &mut Context<'a>) -> EvaluateResult<'a> {
    Ok(Some(Value::List(evaluate_values(elements, context)?)))
}

fn evaluate_index<'a>(
    identifier: &Identifier<'a>,
    index: &Expression<'a>,
    context: &mut Context<'a>,
) -> EvaluateResult<'a> {
    let list = context.variable(identifier)?;
    let index = evaluate(index, context)?.ok_or(SplashRuntimeError::NoValue)?;

    let list = match list {
        Value::List(list) => list,
        value => return Err(SplashRuntimeError::NotAList(value)),
    };

    let index = match index {
        Value::Number(number) => number as usize,
        value => return Err(SplashRuntimeError::NotAnIndex(value)),
    };

    Ok(Some(
        list.get(index)
            .ok_or(SplashRuntimeError::OutOufRange(Value::Number(index as f64)))?
            .clone(),
    ))
}

pub fn evaluate<'a>(expression: &Expression<'a>, context: &mut Context<'a>) -> EvaluateResult<'a> {
    match expression {
        Expression::Atom(atom) => evaluate_atom(atom, context),
        Expression::Operation(operator, operands) => {
            evaluate_operation(operator, operands, context)
        }
        Expression::Function(identifier, parameters) => {
            evaluate_function(identifier, parameters, context)
        }
        Expression::List(elements) => evaluate_list(elements, context),
        Expression::Index(identifier, index) => evaluate_index(identifier, index, context),
    }
}
