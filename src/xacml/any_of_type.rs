use crate::utils::*;
use super::*;

/// 5.7 AnyOf element
/// Shall contain a disjunctive sequence of <AllOf> elements, to be applicable all of these have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AnyOfType {
    #[serde(rename = "AllOf")]
    all_of: Vec<AllOfType>                  // Data type for elements, of which all must match the context to be applicable; if empty, the anyOf is always applicable; might be changed to a simple vec, that can also be of length 0
}

impl AnyOfType {
    pub fn match_request(&self, request: &RequestType) -> Result<TargetResult, XacmlError> {
        if self.all_of.is_empty() {
            return Err(XacmlError::new(XacmlErrorType::FormatError, "AnyOf type contains no AllOf elements".to_string()))
        }
        let all_of_results = self.all_of.iter()
            .map(|a| a.match_request(request))
            .collect::<Result<Vec<TargetResult>, XacmlError>>()?;
        if all_of_results.iter()
            .any(|r| *r == TargetResult::Match)
        {
            return Ok(TargetResult::Match)
        }
        else if all_of_results.iter()
            .all(|r| *r == TargetResult::NoMatch)
        {
            return Ok(TargetResult::NoMatch)
        }
        else 
        {
            return Ok(TargetResult::Indeterminate)    
        }
    }
}