use std::fmt::format;

use super::*;
use crate::utils::*;


/// 5.14 Policy element
/// Describes a policy as smallest unit useable by a PDP
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
#[serde(rename = "Policy")]
pub struct PolicyType {
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xmlns: Option<String>,
    #[serde(rename = "@xmlns:xsi", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    xmlns_xsi: Option<String>,
    #[serde(rename = "@PolicyId")]
    policy_id: String,                          // More specific of URI type
    #[serde(rename = "@Version")]
    version: VersionType,
    #[serde(rename = "@RuleCombiningAlgId")]
    rule_combining_alg_id: RuleCombiningAlgorithms,              // Combining algorithm, as of now string, might later be an enum
    #[serde(rename = "@MaxDelegationDepth", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    max_delegation_depth: Option<i32>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,                
    #[serde(rename = "PolicyIssuer", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_issuer: Option<PolicyIssuerType>,
    #[serde(rename = "PolicyDefaults", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    policy_defaults: Option<DefaultsType>,
    #[serde(rename = "CombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    combiner_parameters: Option<Vec<CombinerParametersType>>,       // own type?
    #[serde(rename = "RuleCombinerParameters", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    rule_combiner_parameters: Option<Vec<RuleCombinerParametersType>>,  // own type?
    #[serde(rename = "Target", default, serialize_with = "serialize_target")]
    target: Option<TargetType>,
    #[serde(rename = "VariableDefinition", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    variable_definition: Option<Vec<String>>,       // own type?
    #[serde(rename = "Rule")]
    rule: Vec<RuleType>,
    #[serde(rename = "ObligationExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    obligation_expressions: Option<Vec<ObligationExpressionsType>>,
    #[serde(rename = "AdvideExpressions", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    advice_expressions: Option<Vec<AdviceExpressionsType>>,
    
}

fn serialize_target<S>(target: &Option<TargetType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if target.is_none() {
        serializer.serialize_str("") 
    } else {
        target.serialize(serializer)
    }
}

impl PolicyType {

    pub fn match_request(&self, request: &RequestType) -> Result<TargetResult, XacmlError> {
        self.target.as_ref().unwrap_or(&TargetType{any_of: None}).match_request(request)
    }

    pub fn evaluate_policy(&self, request: &RequestType) -> Result<DecisionType, XacmlError> {
        let mut result: RuleResult ;
        let mut reason= "Condition";
        let target_result = self.target.as_ref().unwrap_or(&TargetType{any_of: None}).match_request(request)?;
        if target_result == TargetResult::NoMatch
        {
            result = RuleResult::NotApplicable;
            reason = "Target";
        }
        
        todo!()
    }
}