use super::*;

/// 5.25 Expression Substitution Group definition
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
//#[serde(untagged)]
pub enum ExpressionType {
    #[serde(rename = "Apply")]
    Apply(ApplyType),
    #[serde(rename = "AttributeDesignator")]
    AttributeDesignator(AttributeDesignatorType),
    #[serde(rename = "AttributeSelector")]
    AttributeSelector(AttributeSelectorType),
    #[serde(rename = "VariableReference")]
    VariableReference(VariableReferenceType),
    #[serde(rename = "Function")]
    Function(FunctionType),
    #[serde(rename = "AttributeValue")]
    AttributeValue(AttributeValueType)
}

impl ExpressionType {
    pub fn evaluate(&self, request: &RequestType) -> Result<Value, XacmlError> {
        match self {
            ExpressionType::Apply(apply) => apply.evaluate(request),
            ExpressionType::AttributeValue(attribute_value) => attribute_value.evaluate(request),
            ExpressionType::AttributeDesignator(attribute_designator) => attribute_designator.evaluate(request),
            _ => Err(XacmlError::new(XacmlErrorType::NotImplemented, "Expression evaluation not implemented".to_string()))
        }
    }
}
