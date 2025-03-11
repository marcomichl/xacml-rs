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
    pub fn match_request(&self, request: &RequestType) -> Result<bool, XacmlError> {
        for all_of in &self.all_of {
            if all_of.match_request(request)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}