use std::{fmt, str::FromStr};
use serde::{Serialize, Serializer, Deserialize, Deserializer};

use super::*;
use crate::utils::XacmlError;

/// 10.2.3 Algorithm definition
#[allow(dead_code)] 
// During development it does not make sense to have dead code warngings here
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCombiningAlgorithms {
    DenyOverrides, 
    PermitOverrides,
    FirstApplicable,
    OrderedDenyOverrides,
    OrderedPermitOverrides,
    DenyUnlessPermit,
    PermitUnlessDeny,
    LegacyDenyOverrides,
    LegacyPermitOverrides,
    LegacyOrderedDenyOverrides,
    LegacyOrderedPermitOverrides
}

const RULE_COMBINER_MAPPING: &[(RuleCombiningAlgorithms, &str)] = &[
    (RuleCombiningAlgorithms::DenyOverrides, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:deny-overrides" ),
    (RuleCombiningAlgorithms::PermitOverrides, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:permit-overrides" ),
    (RuleCombiningAlgorithms::FirstApplicable, "urn:oasis:names:tc:xacml:1.0:rule-combining-algorithm:first-applicable"),
    (RuleCombiningAlgorithms::OrderedDenyOverrides, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:ordered-deny-overrides"),
    (RuleCombiningAlgorithms::OrderedPermitOverrides, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:ordered-permit-overrides"),
    (RuleCombiningAlgorithms::DenyUnlessPermit, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:deny-unless-permit"),
    (RuleCombiningAlgorithms::PermitUnlessDeny, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:permit-unless-deny"),
    (RuleCombiningAlgorithms::LegacyDenyOverrides, "urn:oasis:names:tc:xacml:1.0:rule-combining-algorithm:deny-overrides"),
    (RuleCombiningAlgorithms::LegacyPermitOverrides, "urn:oasis:names:tc:xacml:1.0:rule-combining-algorithm:permit-overrides"),
    (RuleCombiningAlgorithms::LegacyOrderedDenyOverrides, "urn:oasis:names:tc:xacml:1.1:rule-combining-algorithm:ordered-deny-overrides"),
    (RuleCombiningAlgorithms::LegacyOrderedPermitOverrides, "urn:oasis:names:tc:xacml:1.1:rule-combining-algorithm:ordered-permit-overrides")
];

impl FromStr for RuleCombiningAlgorithms {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        RULE_COMBINER_MAPPING
            .iter()
            .find(|(_, v)| *v == s)
            .map(|(k, _)| *k)
            .ok_or(())
    }
}

impl RuleCombiningAlgorithms {
    pub fn to_xacml_id(&self) -> &str {
        RULE_COMBINER_MAPPING
            .iter()
            .find(|(k, _)| *k == *self)
            .map(|(_, v)| *v)
            .unwrap()   // This should never happen as we have a complete mapping
    }

    pub fn apply(&self, results: &Vec<RuleResult>, parameters: &Option<Vec<RuleCombinerParametersType>>) -> Result<PolicyResult, XacmlError> {
        match self {
            RuleCombiningAlgorithms::DenyOverrides => deny_overrides(results),
            _ => Err(XacmlError::new(crate::utils::XacmlErrorType::NotImplemented, format!("RuleCombiningAlgorithm {} not yet implemented!", self.to_string())))
        }
    }
}

/// Following Appendix C.2 for deny-overrides
fn deny_overrides(results: &Vec<RuleResult>) -> Result<PolicyResult, XacmlError> {
    if results.iter().any(|f| f == &RuleResult::Deny) {
        return Ok(PolicyResult::Deny)
    }
    if results.iter().any(|f| f == &RuleResult::IndeterminateDP) {
        return Ok(PolicyResult::IndeterminateDP)
    }
    if results.iter().any(|f| f == &RuleResult::IndeterminateD) && 
        (results.iter().any(|f| f == &RuleResult::IndeterminateP) || 
        results.iter().any(|f| f == &RuleResult::Permit)) {
        return Ok(PolicyResult::IndeterminateDP)
    }
    if results.iter().any(|f| f == &RuleResult::IndeterminateD) {
        return Ok(PolicyResult::IndetermianteD)
    }
    if results.iter().any(|f| f == &RuleResult::Permit) {
        return Ok(PolicyResult::Permit)
    }
    if results.iter().any(|f| f == &RuleResult::IndeterminateP) {
        return Ok(PolicyResult::IndeterminateP)
    }
    return Ok(PolicyResult::NotApplicable)
}

impl Serialize for RuleCombiningAlgorithms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_xacml_id())
    }
}

impl<'de> Deserialize<'de> for RuleCombiningAlgorithms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        RuleCombiningAlgorithms::from_str(s).map_err( |_| serde::de::Error::custom("invalid RuleCombiningAlgorithm URI"))
    }
}

impl fmt::Display for RuleCombiningAlgorithms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const POLICY_COMBINER_MAPPING: &[(PolicyCombiningAlgorithms, &str)] = &[
    (PolicyCombiningAlgorithms::DenyOverrides, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:deny-overrides"),
    (PolicyCombiningAlgorithms::PermitOverrides, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:permit-overrides"),
    (PolicyCombiningAlgorithms::FirstApplicable, "urn:oasis:names:tc:xacml:1.0:policy-combining-algorithm:first-applicable"),
    (PolicyCombiningAlgorithms::OnlyOneApplicable, "urn:oasis:names:tc:xacml:1.0:policy-combining-algorithm:only-one-applicable"),
    (PolicyCombiningAlgorithms::OrderedDenyOverrides, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:ordered-deny-overrides"),
    (PolicyCombiningAlgorithms::OrderedPermitOverrides, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:ordered-permit-overrides"),
    (PolicyCombiningAlgorithms::DenyUnlessPermit, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:deny-unless-permit"),
    (PolicyCombiningAlgorithms::PermitUnlessDeny, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:permit-unless-deny"),
    (PolicyCombiningAlgorithms::LegacyDenyOverrides, "urn:oasis:names:tc:xacml:1.0:policy-combining-algorithm:deny-overrides"),
    (PolicyCombiningAlgorithms::LegacyPermitOverrides, "urn:oasis:names:tc:xacml:1.0:policy-combining-algorithm:permit-overrides"),
    (PolicyCombiningAlgorithms::LegacyOrderedDenyOverrides, "urn:oasis:names:tc:xacml:1.1:policy-combining-algorithm:ordered-deny-overrides"),
    (PolicyCombiningAlgorithms::LegacyOrderedPermitOverrides, "urn:oasis:names:tc:xacml:1.1:policy-combining-algorithm:ordered-permit-overrides")
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyCombiningAlgorithms {
    DenyOverrides,
    PermitOverrides,
    FirstApplicable,
    OnlyOneApplicable,
    OrderedDenyOverrides,
    OrderedPermitOverrides,
    DenyUnlessPermit,
    PermitUnlessDeny,
    LegacyDenyOverrides,
    LegacyPermitOverrides,
    LegacyOrderedDenyOverrides,
    LegacyOrderedPermitOverrides
}

impl FromStr for PolicyCombiningAlgorithms {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        POLICY_COMBINER_MAPPING
            .iter()
            .find(|(_, v)| *v == s)
            .map(|(k, _)| *k)
            .ok_or(())
    }
}

impl PolicyCombiningAlgorithms {
    pub fn to_xacml_id(&self) -> &str {
        POLICY_COMBINER_MAPPING
            .iter()
            .find(|(k, _)| *k == *self)
            .map(|(_, v)| *v)
            .unwrap()   // This should never happen as we have a complete mapping
    }
}

impl Serialize for PolicyCombiningAlgorithms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_xacml_id())
    }
}

impl<'de> Deserialize<'de> for PolicyCombiningAlgorithms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        PolicyCombiningAlgorithms::from_str(s).map_err( |_| serde::de::Error::custom("invalid function URI"))
    }
}

impl fmt::Display for PolicyCombiningAlgorithms {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

mod test_combining_algorithm{
    #[cfg(test)]
    use super::*;
    use serde::{Serialize, Deserialize};
    #[allow(unused_imports)]
    use quick_xml::{se::to_string, de::from_str};

    #[derive(Serialize, Deserialize)]
    struct RuleTestPolicy {
        #[serde(rename = "RuleCombiningAlgorithmId")]
        pub rule_combining_alg: super::RuleCombiningAlgorithms,
    }

    #[derive(Serialize, Deserialize)]
    struct PolicyTestPolicy {
        #[serde(rename = "PolicyCombiningAlgorithmId")]
        pub policy_combining_alg: super::PolicyCombiningAlgorithms,
    }

    #[test]
    fn test_rule_combining_algorithm() {
        let rule_combining_algorithm = RuleCombiningAlgorithms::DenyOverrides;
        let xacml_id = rule_combining_algorithm.to_xacml_id();
        assert_eq!(xacml_id, "urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:deny-overrides");
        let rule_combining_algorithm = RuleCombiningAlgorithms::from_str(xacml_id).unwrap();
        assert_eq!(rule_combining_algorithm, RuleCombiningAlgorithms::DenyOverrides);
    }

    #[test]
    fn test_policy_combining_algorithm() {
        let policy_combining_algorithm = PolicyCombiningAlgorithms::DenyOverrides;
        let xacml_id = policy_combining_algorithm.to_xacml_id();
        assert_eq!(xacml_id, "urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:deny-overrides");
        let policy_combining_algorithm = PolicyCombiningAlgorithms::from_str(xacml_id).unwrap();
        assert_eq!(policy_combining_algorithm, PolicyCombiningAlgorithms::DenyOverrides);
    }

    #[test]
    fn test_serialize_rule()
    {
        let policy = RuleTestPolicy {
            rule_combining_alg: RuleCombiningAlgorithms::DenyUnlessPermit,
        };
        let xml = to_string(&policy).unwrap();
        assert_eq!(xml, r#"<RuleTestPolicy><RuleCombiningAlgorithmId>urn:oasis:names:tc:xacml:3.0:rule-combining-algorithm:deny-unless-permit</RuleCombiningAlgorithmId></RuleTestPolicy>"#);
    }

    #[test]
    fn test_deserialize_rule()
    {
        let xml = r#"<RuleTestPolicy><RuleCombiningAlgorithmId>urn:oasis:names:tc:xacml:1.0:rule-combining-algorithm:first-applicable</RuleCombiningAlgorithmId></RuleTestPolicy>"#;
        let policy: RuleTestPolicy = from_str(xml).unwrap();
        assert_eq!(policy.rule_combining_alg, RuleCombiningAlgorithms::FirstApplicable);
    }

    #[test]
    fn test_invalid_xml_deserialization_rule() {
        let xml = r#"<RuleTestPolicy><RuleCombiningAlgorithmId>urn:oasis:names:tc:xacml:1.0:rule-combining-algorithm:last-applicable</RuleCombiningAlgorithmId></RuleTestPolicy>"#;
        let result: Result<RuleTestPolicy, _> = from_str(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_policy()
    {
        let policy = PolicyTestPolicy {
            policy_combining_alg: PolicyCombiningAlgorithms::DenyUnlessPermit,
        };
        let xml = to_string(&policy).unwrap();
        assert_eq!(xml, r#"<PolicyTestPolicy><PolicyCombiningAlgorithmId>urn:oasis:names:tc:xacml:3.0:policy-combining-algorithm:deny-unless-permit</PolicyCombiningAlgorithmId></PolicyTestPolicy>"#);
    }

    #[test]
    fn test_deserialize_policy()
    {
        let xml = r#"<PolicyTestPolicy><PolicyCombiningAlgorithmId>urn:oasis:names:tc:xacml:1.0:policy-combining-algorithm:first-applicable</PolicyCombiningAlgorithmId></PolicyTestPolicy>"#;
        let policy: PolicyTestPolicy = from_str(xml).unwrap();
        assert_eq!(policy.policy_combining_alg, PolicyCombiningAlgorithms::FirstApplicable);
    }

    #[test]
    fn test_invalid_xml_deserialization_policy() {
        let xml = r#"<PolicyTestPolicy><PolicyCombiningAlgorithmId>urn:oasis:names:tc:xacml:1.0:policy-combining-algorithm:last-applicable</PolicyCombiningAlgorithmId></PolicyTestPolicy>"#;
        let result: Result<PolicyTestPolicy, _> = from_str(xml);
        assert!(result.is_err());
    }

}