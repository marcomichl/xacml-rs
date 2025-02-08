

/// 10.2.3 Algorithm definition
#[allow(dead_code)] 
// During development it does not make sense to have dead code warngings here
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