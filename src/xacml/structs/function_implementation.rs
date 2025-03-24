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