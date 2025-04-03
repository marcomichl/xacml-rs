use super::*;

pub (super) fn string_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    if value1.len() != 1 || value2.len() != 1 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two parameters that evaluate to a single value".to_string()))
    }

    match (&value1[0], &value2[0]) {
        (Value::String(s1), Value::String(s2)) => {
            return Ok([Value::Boolean(s1 == s2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "StringEquals function requires two string parameters".to_string()))
    }
}

pub (super) fn boolean_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    if value1.len() != 1 || value2.len() != 1 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two parameters that evaluate to a single value".to_string()))
    }

    match (&value1[0], &value2[0]) {
        (Value::Boolean(b1), Value::Boolean(b2)) => {
            return Ok([Value::Boolean(b1 == b2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "BooleanEquals function requires two boolean parameters".to_string()))
    }
}

pub (super) fn integer_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    if value1.len() != 1 || value2.len() != 1 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two parameters that evaluate to a single value".to_string()))
    }

    match (&value1[0], &value2[0]) {
        (Value::Integer(i1), Value::Integer(i2)) => {
            return Ok([Value::Boolean(i1 == i2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerEquals function requires two integer parameters".to_string()))
    }
}

pub (super) fn double_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    if value1.len() != 1 || value2.len() != 1 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two parameters that evaluate to a single value".to_string()))
    }

    match (&value1[0], &value2[0]) {
        (Value::Double(d1), Value::Double(d2)) => {
            return Ok([Value::Boolean(d1 == d2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "DoubleEquals function requires two double parameters".to_string()))
    }
}

pub (super) fn any_uri_equal(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
    if parameters.len() != 2 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two parameters".to_string()))
    }
    let value1 = parameters[0].evaluate(request)?;
    let value2 = parameters[1].evaluate(request)?;

    if value1.len() != 1 || value2.len() != 1 {
        return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two parameters that evaluate to a single value".to_string()))
    }

    match (&value1[0], &value2[0]) {
        (Value::String(s1), Value::String(s2)) => {
            return Ok([Value::Boolean(s1 == s2)].to_vec())
        },
        _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "AnyURIEquals function requires two string parameters".to_string()))
    }
}

pub (super) fn integer_add(parameters: &Vec<ExpressionType>, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
    let mut values: Vec<i64> = [].to_vec();
    for parameter in parameters {
        let value = parameter.evaluate(request)?;
        match (&value[0]) {
            (Value::Integer(int)) => values.push(*int),
            _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "IntegerAdd function requires only integer parameters".to_string()))
        }
    };
    let mut result: i64 = 0;
    values.iter().for_each(|x| result+=*x);
    return Ok(vec![Value::Integer(result)])
}

#[cfg(test)]
mod function_implementation_test {
    use super::*;

    #[test]
    fn integer_add_test() {
        let parameters = vec![ExpressionType::AttributeValue(AttributeValueType{data_type: DataType::Integer, value: Value::Integer(23)}),
            ExpressionType::AttributeValue(AttributeValueType{data_type: DataType::Integer, value: Value::Integer(27)})];
        let request = RequestTypeBuilder::default()
            .return_policy_id_list(true)
            .combined_decision(true)
            .attributes(Vec::<AttributesType>::new())
            .build().unwrap();
        let result = integer_add(&parameters, &request).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(result[0], Value::Integer(50));
    }
}