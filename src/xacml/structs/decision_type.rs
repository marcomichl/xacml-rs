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

impl From<RuleResult> for DecisionType {
    fn from(rule_result: RuleResult) -> Self {
        match rule_result {
            RuleResult::Permit => DecisionType::Permit,
            RuleResult::Deny => DecisionType::Deny,
            RuleResult::IndeterminateDP => DecisionType::Indeterminate,
            RuleResult::IndetermianteD => DecisionType::Indeterminate,
            RuleResult::IndeterminateP => DecisionType::Indeterminate,
            RuleResult::NotApplicable => DecisionType::NotApplicable,
        }
    }
}

impl From<PolicyResult> for DecisionType {
    fn from(rule_result: PolicyResult) -> Self {
        match rule_result {
            PolicyResult::Permit => DecisionType::Permit,
            PolicyResult::Deny => DecisionType::Deny,
            PolicyResult::IndeterminateDP => DecisionType::Indeterminate,
            PolicyResult::IndetermianteD => DecisionType::Indeterminate,
            PolicyResult::IndeterminateP => DecisionType::Indeterminate,
            PolicyResult::NotApplicable => DecisionType::NotApplicable,
        }
    }
}