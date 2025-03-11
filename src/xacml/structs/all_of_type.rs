use super::*;
use crate::utils::*;

/// 5.8 AllOf element
/// Shall contain a conjunctive sequence of <Match> elements, to be applicable all of these have to match the decision request.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct AllOfType {
    #[serde(rename = "Match")]
    _match: Vec<MatchType>                           // One or many, all of which must match the context to be applicable
}

impl AllOfType {
    pub fn match_request(&self, request: &RequestType) -> Result<bool, XacmlError> {
        for match_type in &self._match {
            if !match_type.match_request(request)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}