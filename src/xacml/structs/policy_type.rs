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
    rule_combiner_parameters: Option<Vec<String>>,  // own type?
    #[serde(rename = "Target", default, serialize_with = "serialize_target")]
    target: Vec<TargetType>,
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

fn serialize_target<S>(targets: &Vec<TargetType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if targets.is_empty() {
        serializer.serialize_str("") 
    } else {
        targets.serialize(serializer)
    }
}

impl PolicyType {
    pub fn get_target(&self) -> &Vec<TargetType> {
        &self.target
    }

    pub fn match_request(&self, request: &RequestType) -> Result<bool, XacmlError> {
        for target in &self.target {
            if target.match_request(request)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}