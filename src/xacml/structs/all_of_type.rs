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
    pub fn match_request(&self, request: &RequestType) -> Result<TargetResult, XacmlError> {
        if self._match.is_empty() {
            return Err(XacmlError::new(XacmlErrorType::FormatError, "AllOf type contains no match elements".to_string()))
        }
        let match_results = self._match.iter()
            .map(|m| m.match_request(request))
            .collect::<Result<Vec<TargetResult>, XacmlError>>()?;   // uses the FromIterator trait included in Result
        if match_results.iter()
            .all(|r| *r == TargetResult::Match) 
        {
            return Ok(TargetResult::Match)
        }
        else if match_results.iter()
            .any(|r| *r == TargetResult::NoMatch)
        {
            return Ok(TargetResult::NoMatch)
        }
        else 
        {
            return Ok(TargetResult::Indeterminate)
        }
    }
}