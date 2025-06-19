use super::*;

/// 5.27 ApplyType definition
/// Describes the application of a function to its arguments
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ApplyType {
    #[serde(rename = "@FunctionId")]
    function_id: FunctionId,        // More specific of URI type
    #[serde(rename = "@Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "$value",skip_serializing_if = "Option::is_none")]
    #[builder(default)] 
    expression: Option<Vec<ExpressionType>>
}

impl ApplyType {
    /// Evaluate the apply
    pub fn evaluate(&self, request: &RequestType) -> Result<Vec<Value>, XacmlError> {
        let empty_vec: Vec<ExpressionType> = Vec::new();
        let parameters: Vec<Value> = self.expression.as_ref()
            .unwrap_or_else(|| &empty_vec)
            .iter()
            .map(|p| p.evaluate(request))
            .collect::<Result<Vec<Vec<Value>>, XacmlError>>()?
            .into_iter()
            .flatten()
            .collect();
        //return self.function_id.apply(self.expression.as_ref().unwrap_or_else(|| &empty_vec), request)
        self.function_id.apply_function(parameters.iter().collect())
    }
}