use super::{evaluate::Result, value::Value, SplashRuntimeError};

/*************
 * OPERATORS *
 *************/

pub fn not(value: Value) -> Result<'static> {
    match value {
        Value::Boolean(boolean) => Ok(Some(Value::Boolean(!boolean))),
        value => Err(SplashRuntimeError::InvalidSignatureType("not", vec![value])),
    }
}

pub fn plus(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Number(left + right))),
        (Value::String(left), Value::String(right)) => {
            let mut result = left;
            result.push_str(&right);
            Ok(Some(Value::String(result)))
        }
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "plus",
            vec![left, right],
        )),
    }
}

pub fn minus(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Number(left - right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "minus",
            vec![left, right],
        )),
    }
}

pub fn times(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Number(left * right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "times",
            vec![left, right],
        )),
    }
}

pub fn divide(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            if right == 0.0 {
                Err(SplashRuntimeError::DivisionByZero)
            } else {
                Ok(Some(Value::Number(left / right)))
            }
        }
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "divide",
            vec![left, right],
        )),
    }
}

pub fn modulo(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            if right == 0.0 {
                Err(SplashRuntimeError::DivisionByZero)
            } else {
                Ok(Some(Value::Number(left % right)))
            }
        }
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "modulo",
            vec![left, right],
        )),
    }
}

pub fn equal(left: Value, right: Value) -> Result<'static> {
    Ok(Some(Value::Boolean(left == right)))
}

pub fn not_equal(left: Value, right: Value) -> Result<'static> {
    Ok(Some(Value::Boolean(left != right)))
}

pub fn greater_or_equal(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left >= right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "greater_or_equal",
            vec![left, right],
        )),
    }
}

pub fn greater_than(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left > right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "greater_than",
            vec![left, right],
        )),
    }
}

pub fn less_or_equal(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left <= right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "less_or_equal",
            vec![left, right],
        )),
    }
}

pub fn less_than(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left < right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "less_than",
            vec![left, right],
        )),
    }
}

pub fn and(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Boolean(left), Value::Boolean(right)) => Ok(Some(Value::Boolean(left && right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "and",
            vec![left, right],
        )),
    }
}

pub fn or(left: Value, right: Value) -> Result<'static> {
    match (left, right) {
        (Value::Boolean(left), Value::Boolean(right)) => Ok(Some(Value::Boolean(left || right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "or",
            vec![left, right],
        )),
    }
}

/*************
 * FUNCTIONS *
 *************/

pub fn print(arguments: &[Value]) -> Result<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "print",
            1,
            arguments.len(),
        ));
    }

    println!("{}", arguments[0]);
    Ok(None)
}

pub fn string(arguments: &[Value]) -> Result<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "string",
            1,
            arguments.len(),
        ));
    }

    Ok(Some(Value::String(format!("{}", arguments[0]))))
}

pub fn length(arguments: &[Value]) -> Result<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "length",
            1,
            arguments.len(),
        ));
    }

    let len = match &arguments[0] {
        Value::List(list) => list.len(),
        Value::String(string) => string.len(),
        value => return Err(SplashRuntimeError::NotAList(value.clone())),
    };

    Ok(Some(Value::Number(len as f64)))
}

pub fn push(arguments: &[Value]) -> Result<'static> {
    if arguments.len() != 2 {
        return Err(SplashRuntimeError::InvalidSignature(
            "push",
            2,
            arguments.len(),
        ));
    }

    let mut list = match &arguments[0] {
        Value::List(list) => list.clone(),
        value => return Err(SplashRuntimeError::NotAList(value.clone())),
    };

    list.push(arguments[1].clone());
    Ok(Some(Value::List(list)))
}

pub fn pop(arguments: &[Value]) -> Result<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "pop",
            1,
            arguments.len(),
        ));
    }

    let mut list = match &arguments[0] {
        Value::List(list) => list.clone(),
        value => return Err(SplashRuntimeError::NotAList(value.clone())),
    };

    list.pop();
    Ok(Some(Value::List(list)))
}

pub fn range(arguments: &[Value]) -> Result<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "range",
            1,
            arguments.len(),
        ));
    }

    let size = match &arguments[0] {
        Value::Number(number) => *number as usize,
        _ => {
            return Err(SplashRuntimeError::InvalidSignatureType(
                "range",
                arguments.to_vec(),
            ))
        }
    };

    let range = (0..size).map(|i| Value::Number(i as f64)).collect();
    Ok(Some(Value::List(range)))
}
