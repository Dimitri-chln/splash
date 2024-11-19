use std::collections::{hash_map::Entry, HashMap};

use crate::parse::{Block, Identifier};

use super::{builtin, function::Function, value::Value, SplashRuntimeError};

pub struct Context<'a> {
    variables: Vec<HashMap<Identifier<'a>, Value>>,
    functions: Vec<HashMap<Identifier<'a>, Function<'a>>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            variables: vec![HashMap::new()],
            functions: vec![HashMap::new()],
        }
    }

    pub fn variable(&self, identifier: Identifier<'a>) -> Result<Value, SplashRuntimeError<'a>> {
        for scope in self.variables.iter().rev() {
            match scope.get(identifier) {
                Some(value) => return Ok(value.clone()),
                None => continue,
            }
        }

        return Err(SplashRuntimeError::NotDefined(identifier));
    }

    pub fn function(
        &self,
        identifier: Identifier<'a>,
    ) -> Result<&Function<'a>, SplashRuntimeError<'a>> {
        for scope in self.functions.iter().rev() {
            match scope.get(identifier) {
                Some(value) => return Ok(value),
                None => continue,
            }
        }

        match identifier {
            "print" => Ok(&Function::BuiltIn(builtin::print)),
            "string" => Ok(&Function::BuiltIn(builtin::string)),
            _ => Err(SplashRuntimeError::NotDefined(identifier)),
        }
    }

    pub fn initialize_variable(&mut self, identifier: Identifier<'a>, value: Value) {
        self.variables.last_mut().unwrap().insert(identifier, value);
    }

    pub fn assign_variable(
        &mut self,
        identifier: Identifier<'a>,
        value: Value,
    ) -> Result<(), SplashRuntimeError<'a>> {
        for scope in self.variables.iter_mut().rev() {
            if let Entry::Occupied(mut entry) = scope.entry(identifier) {
                entry.insert(value);
                return Ok(());
            }
        }

        return Err(SplashRuntimeError::NotDefined(identifier));
    }

    pub fn initialize_function(
        &mut self,
        identifier: Identifier<'a>,
        arguments: Vec<Identifier<'a>>,
        body: Block<'a>,
    ) {
        self.functions
            .last_mut()
            .unwrap()
            .insert(identifier, Function::Custom(arguments, body));
    }

    pub fn child<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.variables.push(HashMap::new());
        self.functions.push(HashMap::new());

        let result = f(self);

        self.variables.pop();
        self.functions.pop();

        result
    }
}
