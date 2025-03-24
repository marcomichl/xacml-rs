use super::*;

pub (super) fn string_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Value, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    match (value1, value2) {
        (Value::String(s1), Value::String(s2)) => {
            return Ok(Value::Boolean(s1 == s2))
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two string parameters".to_string()))
    }
}

pub (super) fn boolean_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Value, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    match (value1, value2) {
        (Value::Boolean(b1), Value::Boolean(b2)) => {
            return Ok(Value::Boolean(b1 == b2))
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two boolean parameters".to_string()))
    }
}

pub (super) fn integer_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Value, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    match (value1, value2) {
        (Value::Integer(i1), Value::Integer(i2)) => {
            return Ok(Value::Boolean(i1 == i2))
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two integer parameters".to_string()))
    }
}

pub (super) fn double_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Value, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    match (value1, value2) {
        (Value::Double(d1), Value::Double(d2)) => {
            return Ok(Value::Boolean(d1 == d2))
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two double parameters".to_string()))
    }
}

pub (super) fn any_uri_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Value, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    match (value1, value2) {
        (Value::String(s1), Value::String(s2)) => {
            return Ok(Value::Boolean(s1 == s2))
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two string parameters".to_string()))
    }
}