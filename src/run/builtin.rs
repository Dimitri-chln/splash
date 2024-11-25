use super::{evaluate::EvaluateResult, value::Value, SplashRuntimeError};

/*************
 * OPERATORS *
 *************/

pub fn not(value: Value) -> EvaluateResult<'static> {
    match value {
        Value::Boolean(boolean) => Ok(Some(Value::Boolean(!boolean))),
        value => Err(SplashRuntimeError::InvalidSignatureType("not", vec![value])),
    }
}

pub fn plus(left: Value, right: Value) -> EvaluateResult<'static> {
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

pub fn minus(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Number(left - right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "minus",
            vec![left, right],
        )),
    }
}

pub fn times(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Number(left * right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "times",
            vec![left, right],
        )),
    }
}

pub fn divide(left: Value, right: Value) -> EvaluateResult<'static> {
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

pub fn modulo(left: Value, right: Value) -> EvaluateResult<'static> {
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

pub fn equal(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            Ok(Some(Value::Boolean((left - right).abs() <= f64::EPSILON)))
        }
        (Value::Boolean(left), Value::Boolean(right)) => Ok(Some(Value::Boolean(left == right))),
        (Value::String(left), Value::String(right)) => Ok(Some(Value::Boolean(left == right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "equal",
            vec![left, right],
        )),
    }
}

pub fn not_equal(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            Ok(Some(Value::Boolean((left - right).abs() > f64::EPSILON)))
        }
        (Value::Boolean(left), Value::Boolean(right)) => Ok(Some(Value::Boolean(left != right))),
        (Value::String(left), Value::String(right)) => Ok(Some(Value::Boolean(left != right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "not_equal",
            vec![left, right],
        )),
    }
}

pub fn greater_than(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left > right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "greater_than",
            vec![left, right],
        )),
    }
}

pub fn greater_or_equal(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left >= right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "greater_or_equal",
            vec![left, right],
        )),
    }
}

pub fn less_than(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left < right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "less_than",
            vec![left, right],
        )),
    }
}

pub fn less_or_equal(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Ok(Some(Value::Boolean(left <= right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "less_or_equal",
            vec![left, right],
        )),
    }
}

pub fn and(left: Value, right: Value) -> EvaluateResult<'static> {
    match (left, right) {
        (Value::Boolean(left), Value::Boolean(right)) => Ok(Some(Value::Boolean(left && right))),
        (left, right) => Err(SplashRuntimeError::InvalidSignatureType(
            "and",
            vec![left, right],
        )),
    }
}

pub fn or(left: Value, right: Value) -> EvaluateResult<'static> {
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

pub fn print(arguments: &[Value]) -> EvaluateResult<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "print",
            1,
            arguments.len(),
        ));
    }

    println!("{}", arguments[0].to_string());
    Ok(None)
}

pub fn string(arguments: &[Value]) -> EvaluateResult<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "str",
            1,
            arguments.len(),
        ));
    }

    Ok(Some(Value::String(arguments[0].to_string())))
}

pub fn length(arguments: &[Value]) -> EvaluateResult<'static> {
    if arguments.len() != 1 {
        return Err(SplashRuntimeError::InvalidSignature(
            "str",
            1,
            arguments.len(),
        ));
    }

    let list = match &arguments[0] {
        Value::List(list) => list,
        value => return Err(SplashRuntimeError::NotAList(value.clone())),
    };

    Ok(Some(Value::Number(list.len() as f64)))
}
