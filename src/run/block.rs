use crate::parse::{Block, Statement};

use super::{
    context::Context, evaluate::evaluate, utils::evaluate_predicate, value::Value,
    SplashRuntimeError,
};

#[derive(Debug)]
pub enum BlockValue {
    Return(Option<Value>),
    None,
}

pub fn run<'a>(
    block: &Block<'a>,
    context: &mut Context<'a>,
) -> Result<BlockValue, SplashRuntimeError<'a>> {
    for statement in block.statements() {
        match statement {
            Statement::Simple(expression) => {
                evaluate(expression, context)?;
            }
            Statement::Block(block) => {
                let value = context.child(|context| self::run(block, context))?;

                match value {
                    BlockValue::Return(value) => return Ok(BlockValue::Return(value)),
                    BlockValue::None => {}
                }
            }
            Statement::Initialization(identifier, expression) => {
                let value = evaluate(expression, context)?.ok_or(SplashRuntimeError::NoValue)?;
                context.initialize_variable(identifier, value);
            }
            Statement::Assignment(identifier, expression) => {
                let value = evaluate(expression, context)?.ok_or(SplashRuntimeError::NoValue)?;
                context.assign_variable(identifier, value)?;
            }
            Statement::IndexAssignment(identifier, index, expression) => {
                let list = context.variable(identifier)?;
                let index = evaluate(index, context)?.ok_or(SplashRuntimeError::NoValue)?;

                let mut list = match list {
                    Value::List(list) => list,
                    value => return Err(SplashRuntimeError::NotAList(value)),
                };

                let index = match index {
                    Value::Number(number) => number as usize,
                    value => return Err(SplashRuntimeError::NotAnIndex(value)),
                };

                let value = evaluate(expression, context)?.ok_or(SplashRuntimeError::NoValue)?;

                if index < list.len() as usize {
                    list[index as usize] = value;
                    context.assign_variable(identifier, Value::List(list))?
                } else {
                    return Err(SplashRuntimeError::OutOufRange(Value::Number(index as f64)));
                }
            }
            Statement::If(predicate, then) => {
                if evaluate_predicate(predicate, context)? {
                    match context.child(|context| self::run(then, context))? {
                        BlockValue::Return(value) => return Ok(BlockValue::Return(value)),
                        BlockValue::None => {}
                    }
                }
            }
            Statement::IfElse(predicate, then, otherwise) => {
                if evaluate_predicate(predicate, context)? {
                    match context.child(|context| self::run(then, context))? {
                        BlockValue::Return(value) => return Ok(BlockValue::Return(value)),
                        BlockValue::None => {}
                    }
                } else {
                    match context.child(|context| self::run(otherwise, context))? {
                        BlockValue::Return(value) => return Ok(BlockValue::Return(value)),
                        BlockValue::None => {}
                    }
                }
            }
            Statement::While(predicate, body) => {
                while evaluate_predicate(predicate, context)? {
                    match context.child(|context| self::run(body, context))? {
                        BlockValue::Return(value) => return Ok(BlockValue::Return(value)),
                        BlockValue::None => {}
                    }
                }
            }
            Statement::For(identifier, list, body) => {
                let list = evaluate(list, context)?.ok_or(SplashRuntimeError::NoValue)?;
                match list {
                    Value::List(list) => {
                        for element in list {
                            let block_value = context.child(|context| {
                                context.initialize_variable(identifier, element);
                                self::run(body, context)
                            })?;
                            match block_value {
                                BlockValue::Return(value) => return Ok(BlockValue::Return(value)),
                                BlockValue::None => {}
                            }
                        }
                    }
                    value => return Err(SplashRuntimeError::NotAList(value)),
                }
            }
            Statement::Return(expression) => {
                return match expression {
                    Some(expression) => Ok(BlockValue::Return(evaluate(expression, context)?)),
                    None => Ok(BlockValue::Return(None)),
                };
            }
            Statement::Definition(identifier, arguments, body) => {
                context.initialize_function(identifier, arguments.clone(), body.clone());
            }
        }
    }

    Ok(BlockValue::None)
}
