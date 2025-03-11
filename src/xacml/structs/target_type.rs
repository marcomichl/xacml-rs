use super::*;
use crate::utils::*;

/// Shall appear in the policy set or the policy elements, may be contained in a rule element
/// Shall contain a conjunctive sequence of <AnyOf> elements, to be applicable one of these has to match the decision request.
/// Each AnyOf element contains a disjunctive AllOf element, that all have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct TargetType {
    #[serde(rename = "AnyOf", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    any_of: Option<Vec<AnyOfType>>                  // Data type for elements, of which one must match the context to be applicable; if empty, the target is always applicable; might be changed to a simple vec, that can also be of length 0
}

impl TargetType {
    pub fn match_request(&self, request: &RequestType) -> Result<bool, XacmlError> {
        if self.any_of.is_none() {
            return Ok(true);
        }
        for any_of in self.any_of.as_ref().unwrap() {   // safe to unwrap as we checked for none
            if !any_of.match_request(request)? {                    // 5.6: "The <Target> element SHALL contain a conjunctive sequence of <AnyOf> elements"
                return Ok(false);
            }
        };
        Ok(true)
    }
}