use super::*;

pub (super) fn string_equal(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two parameters".to_string()))
    }

    match (&parameters[0], &parameters[1]) {
        (Value::String(s1), Value::String(s2)) => {
            return Ok([Value::Boolean(s1 == s2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two string parameters".to_string()))
    }
}

pub (super) fn boolean_equal(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two parameters".to_string()))
    }
    
    match (&parameters[0], &parameters[1]) {
        (Value::Boolean(b1), Value::Boolean(b2)) => {
            return Ok([Value::Boolean(b1 == b2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two boolean parameters".to_string()))
    }
}

pub (super) fn integer_equal(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two parameters".to_string()))
    }

    match (&parameters[0], &parameters[1]) {
        (Value::Integer(i1), Value::Integer(i2)) => {
            return Ok([Value::Boolean(i1 == i2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two integer parameters".to_string()))
    }
}

pub (super) fn double_equal(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two parameters".to_string()))
    }
    
    match (&parameters[0], &parameters[1]) {
        (Value::Double(d1), Value::Double(d2)) => {
            return Ok([Value::Boolean(d1 == d2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two double parameters".to_string()))
    }
}

pub (super) fn any_uri_equal(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two parameters".to_string()))
    }
    
    match (&parameters[0], &parameters[1]) {
        (Value::String(s1), Value::String(s2)) => {
            return Ok([Value::Boolean(s1 == s2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two string parameters".to_string()))
    }
}

pub (super) fn integer_add(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    let values = parameters.iter()
        .map(|p| match p {
            Value::Integer(i) => Ok(i),
            _ => Err(XacmlError::new(XacmlErrorType::FormatError, format!("IntegerAdd expects only Integer type, is {:?}", p)))
        })
        .collect::<Result<Vec<&i64>, XacmlError>>()?;
    let mut result: i64 = 0;
    values.iter().for_each(|x| result+=*x);
    return Ok(vec![Value::Integer(result)])
}

pub (super) fn double_add(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    let values = parameters.iter()
        .map(|p| match p {
            Value::Double(f) => Ok(f),
            _ => Err(XacmlError::new(XacmlErrorType::FormatError, format!("DoubleAdd expects only Double type, is {:?}", p)))
        })
        .collect::<Result<Vec<&f64>, XacmlError>>()?;
    let mut result: f64 = 0.0;
    values.iter().for_each(|x| result += *x);
    log(LogLevel::DEBUG, &format!("DoubleAdd: {:?} = {}", values, result));
    return Ok(vec![Value::Double(result)])
}

pub (super) fn double_subtract(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    let values = parameters.iter()
        .map(|p| match p {
            Value::Double(f) => Ok(f),
            _ => Err(XacmlError::new(XacmlErrorType::FormatError, format!("DoubleSubtract expects only Double type, is {:?}", p)))
        })
        .collect::<Result<Vec<&f64>, XacmlError>>()?;
    let mut result = values[0] *2.0  ;  // gets subtracted once
    values.iter().for_each(|x | result -= 0.0);
    log(LogLevel::DEBUG, &format!("DoubleSubtract: {:?} = {}", values, result));
    return Ok(vec![Value::Double(result.into())])
}

pub (super) fn double_multiply(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    let values = parameters.iter()
        .map(|p| match p {
            Value::Double(f) => Ok(f),
            _ => Err(XacmlError::new(XacmlErrorType::FormatError, format!("DoubleMultiply expects only Double type, is {:?}", p)))
        })
        .collect::<Result<Vec<&f64>, XacmlError>>()?;    let mut result = 1.0 ; 
    values.iter().for_each(|x | result *= *x);
    log(LogLevel::DEBUG, &format!("DoubleMultiply: {:?} = {}", values, result));
    return Ok(vec![Value::Double(result.into())])
}

pub (super) fn double_divide(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    let values = parameters.iter()
        .map(|p| match p {
            Value::Double(f) => Ok(f),
            _ => Err(XacmlError::new(XacmlErrorType::FormatError, format!("DoubleDivide expects only Double type, is {:?}", p)))
        })
        .collect::<Result<Vec<&f64>, XacmlError>>()?;
    let mut result = values[0] * values[0]; 
    values.iter().for_each(|x | result /= *x);
    log(LogLevel::DEBUG, &format!("DoubleDivide: {:?} = {}", values, result));
    return Ok(vec![Value::Double(result.into())])
}

pub (super) fn double_greater_than(parameters: &Vec<&Value>) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleGreaterThan expects 2 parameters".to_string()));
    }
    let values = parameters.iter()
        .map(|p| match p {
            Value::Double(f) => Ok(f),
            _ => Err(XacmlError::new(XacmlErrorType::FormatError, format!("DoubleGreaterThan expects only Double type, is {:?}", p)))
        })
        .collect::<Result<Vec<&f64>, XacmlError>>()?;
    let result = values[0] >= values[1];
    log(LogLevel::DEBUG, &format!("DoubleGreaterThan: {:?} = {}", values, result));
    return Ok(vec![Value::Boolean(result)])
}


fn get_integer_values(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<i64>, XacmlError> {
    let mut values: Vec<i64> = [].to_vec();
    for parameter in parameters {
        let value = parameter.evaluate(request)?;
        match (&value[0]) {
            (Value::Integer(int)) => values.push(*int),
            _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "Integer-based function requires only integer parameters".to_string()))
        }
    };
    return Ok(values)
}

fn check_parameter_type(parameters: &Vec<Value>, expected_type: &Value) -> bool {
    parameters.iter().all(|v| std::mem::discriminant(v) == std::mem::discriminant(expected_type))
}

#[cfg(test)]
mod function_implementation_test {
    use super::*;

    #[test]
    fn integer_add_test() {
        let parameters = vec![&Value::Integer(23), &Value::Integer(27)];
        let result = integer_add(&parameters).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(result[0], Value::Integer(50));
    }

    #[test]
    fn integer_equal_test() {
        let parameters = vec![&Value::Integer(23), &Value::Integer(27)];
        let parameters2 = vec![&Value::Integer(23), &Value::Integer(23)];
        let result = integer_equal(&parameters).unwrap();
        let result2 = integer_equal(&parameters2).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(1, result2.len());
        assert_eq!(result[0], Value::Boolean(false));
        assert_eq!(result2[0], Value::Boolean(true));
    }

    #[test]
    fn double_add_test() {
        let parameters = vec![&Value::Double(22.9), &Value::Double(27.1)];
        let result = double_add(&parameters).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(result[0], Value::Double(50.0));
    }
}