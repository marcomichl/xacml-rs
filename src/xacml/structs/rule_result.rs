#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub (super) enum RuleResult{
    Permit,
    Deny,
    IndeterminateDP,
    IndetermianteD,
    IdeterminateP,
    NotApplicable
}

impl RuleResult {
    pub (super) fn get_legacy_result(&self) -> RuleResult {
        match self {
            RuleResult::IdeterminateP => RuleResult::IndeterminateDP,
            RuleResult::IndetermianteD => RuleResult::IndeterminateDP,
            _ => *self
        }
    }
}