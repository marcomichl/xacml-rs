use super::*;

/// 5.50 - 5.52 are optional and skipped
/// 5.53 DecisionType
/// Enumeration to indicate the decision of a policy
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum DecisionType {
    Permit,
    Deny,
    Indeterminate,
    NotApplicable
}