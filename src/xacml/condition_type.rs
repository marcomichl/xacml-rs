use super::*;
use crate::utils::*;

/// 5.26 ConditionType definition
/// Boolean function over attributes or functions of attributes
/// Not clear if a vector is correct, as a xs:sequence is defined, but description says one expression
/// Might also be flattened?
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct ConditionType {
    #[serde(rename = "$value")]
    expression: ExpressionType
}

/// 7.9 Condition Evaluation
impl ConditionType {
    /// Evaluate the condition
    pub fn evaluate(&self, request: &RequestType) -> Result<bool, XacmlError> {
        let result = self.expression.evaluate(request)?;
        if result.len() != 1 {
            return Err(XacmlError::new(XacmlErrorType::ProcessingError, "Condition evaluation result is not a single value".to_string()))
        }
        match result[0] {
            Value::Boolean(value) => {
                return Ok(value)
            },
            _ => return Err(XacmlError::new(XacmlErrorType::ProcessingError, "Condition evaluation result is not a boolean".to_string()))
        }
    }
}