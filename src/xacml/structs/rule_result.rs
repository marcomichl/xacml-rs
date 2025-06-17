#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub (super) enum RuleResult{
    Permit,
    Deny,
    IndeterminateDP,
    IndeterminateD,
    IndeterminateP,
    NotApplicable
}

impl RuleResult {
    pub (super) fn get_legacy_result(&self) -> RuleResult {
        match self {
            RuleResult::IndeterminateP => RuleResult::IndeterminateDP,
            RuleResult::IndeterminateD => RuleResult::IndeterminateDP,
            _ => *self
        }
    }

}