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
                context.initialize_variable(*identifier, value);
            }
            Statement::Assignment(identifier, expression) => {
                let value = evaluate(expression, context)?.ok_or(SplashRuntimeError::NoValue)?;
                context.assign_variable(*identifier, value)?;
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
            Statement::Return(expression) => {
                return match expression {
                    Some(expression) => Ok(BlockValue::Return(evaluate(expression, context)?)),
                    None => Ok(BlockValue::Return(None)),
                };
            }
            Statement::Definition(identifier, arguments, body) => {
                context.initialize_function(*identifier, arguments.clone(), body.clone());
            }
        }
    }

    Ok(BlockValue::None)
}
