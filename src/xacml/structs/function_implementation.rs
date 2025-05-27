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
    values.iter().for_each(|x | result -= *x);
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


fn check_parameter_type(parameters: &Vec<Value>, expected_type: &Value) -> bool {
    parameters.iter().all(|v| std::mem::discriminant(v) == std::mem::discriminant(expected_type))
}

#[cfg(test)]
mod function_implementation_test {
    use super::*;

    #[test]
    fn string_equal_test() {
        let parameters = vec![Value::String("String".to_string()), Value::String("String".to_string())];
        let parameters2 = vec![Value::String("String".to_string()), Value::String("AnotherString".to_string())];
        let result = string_equal(&parameters.iter().collect()).unwrap();
        let result2 = string_equal(&parameters2.iter().collect()).unwrap();
        assert_eq!(result, vec![Value::Boolean(true)]);
        assert_eq!(result2, vec![Value::Boolean(false)]);
    }

    #[test]
    fn boolean_equal_test() {
        let parameters = vec![&Value::Boolean(true), &Value::Boolean(true)];
        let parameters2 = vec![&Value::Boolean(true), &Value::Boolean(false)];
        let result = boolean_equal(&parameters).unwrap();
        let result2 = boolean_equal(&parameters2).unwrap();
        assert_eq!(result, vec![Value::Boolean(true)]);
        assert_eq!(result2, vec![Value::Boolean(false)]);
    }

    #[test]
    fn integer_equal_test() {
        let parameters = vec![&Value::Integer(45), &Value::Integer(45)];
        let parameters2 = vec![&Value::Integer(45), &Value::Integer(58)];
        let result = integer_equal(&parameters).unwrap();
        let result2 = integer_equal(&parameters2).unwrap();
        assert_eq!(result, vec![Value::Boolean(true)]);
        assert_eq!(result2, vec![Value::Boolean(false)]);
    }

    #[test]
    fn double_equal_test() {
        let parameters = vec![&Value::Double(45.), &Value::Double(45.)];
        let parameters2 = vec![&Value::Double(45.), &Value::Double(45.1)];
        let result = double_equal(&parameters).unwrap();
        let result2 = double_equal(&parameters2).unwrap();
        assert_eq!(result, vec![Value::Boolean(true)]);
        assert_eq!(result2, vec![Value::Boolean(false)]);
    }

    #[test]
    fn integer_add_test() {
        let parameters = vec![&Value::Integer(23), &Value::Integer(27)];
        let result = integer_add(&parameters).unwrap();
        assert_eq!(result, vec![Value::Integer(50)]);
    }

    #[test]
    #[should_panic]
    fn integer_add_fail() {
        let parameters = vec![&Value::Integer(23), &Value::Double(27.0)];
        integer_add(&parameters).unwrap();
    }

    #[test]
    fn double_add_test() {
        let parameters = vec![&Value::Double(22.9), &Value::Double(27.1)];
        let result = double_add(&parameters).unwrap();
        assert_eq!(result, vec![Value::Double(50.0)]);
    }

    #[test]
    fn double_subtract_test() {
        let parameters = vec![&Value::Double(22.), &Value::Double(21.)];
        let result = double_subtract(&parameters).unwrap();
        assert_eq!(result, vec![Value::Double(1.)]);
    }

    #[test]
    fn double_multiply_test() {
        let parameters = vec![&Value::Double(22.9), &Value::Double(27.1)];
        let result = double_multiply(&parameters).unwrap();
        assert_eq!(result, vec![Value::Double(620.59)]);
    }

    #[test]
    fn double_divide_test() {
        let parameters = vec![&Value::Double(34.5), &Value::Double(3.)];
        let result = double_divide(&parameters).unwrap();
        assert_eq!(result, vec![Value::Double(11.5)]);
    }

}