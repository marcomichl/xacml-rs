
/// Enum for the result of a policy
/// Combined by PolicySetCombiningAlgorithms
/// Mapped to a DecisionType in case the PolicyResult shall be returned
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PolicyResult{
    Permit,
    Deny,
    IndeterminateDP,
    IndetermianteD,
    IndeterminateP,
    NotApplicable
}

impl PolicyResult {
    pub (super) fn get_legacy_result(&self) -> PolicyResult {
        match self {
            PolicyResult::IndeterminateP => PolicyResult::IndeterminateDP,
            PolicyResult::IndetermianteD => PolicyResult::IndeterminateDP,
            _ => *self
        }
    }

}