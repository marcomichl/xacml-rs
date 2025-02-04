

/// 10.2.3 Algorithm definition
#[derive(PartialEq, Eq)]
pub enum Algorithms {
    RuleCombiningAlgorithmDenyOverrides, 
    PolicyCombiningAlgorithmDenyOverrides,
    RuleCombiningAlgorithmPermitOverrides,
    PolicyCombiningAlgorithmPermitOverrides,
    RuleCombiningAlgorithmFirstApplicable,
    PolicyCombiningAlgorithmFirstApplicable,
    PolicyCombiningAlgorithmOnlyOneApplicable,
    RuleCombiningAlgorithmOrderedDenyOverrides,
    PolicyCombiningAlgorithmOrderedDenyOverrides,
    RuleCombiningAlgorithmOrderedPermitOverrides,
    PolicyCombiningAlgorithmOrderedPermitOverrides,
    RuleCombiningAlgorithmDenyUnlessPermit,
    PolicyCombiningAlgorithmDenyUnlessPermit,
    RuleCombiningAlgorithmPermitUnlessDeny,
    PolicyCombiningAlgorithmPermitUnlessDeny
}